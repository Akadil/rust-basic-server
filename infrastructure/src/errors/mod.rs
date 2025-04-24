use application::errors::ApplicationError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("JWT error: {0}")]
    JwtError(String),
    
    #[error("Password error: {0}")]
    PasswordError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Application error: {0}")]
    ApplicationError(#[from] ApplicationError),
}

impl From<sqlx::Error> for InfrastructureError {
    fn from(error: sqlx::Error) -> Self {
        Self::DatabaseError(error.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for InfrastructureError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        Self::JwtError(error.to_string())
    }
}

impl From<std::io::Error> for InfrastructureError {
    fn from(error: std::io::Error) -> Self {
        Self::ConfigurationError(error.to_string())
    }
}

impl From<serde_json::Error> for InfrastructureError {
    fn from(error: serde_json::Error) -> Self {
        Self::SerializationError(error.to_string())
    }
}

impl From<InfrastructureError> for ApplicationError {
    fn from(error: InfrastructureError) -> Self {
        match error {
            InfrastructureError::DatabaseError(msg) => ApplicationError::UnexpectedError(format!("Database error: {}", msg)),
            InfrastructureError::ConfigurationError(msg) => ApplicationError::UnexpectedError(format!("Configuration error: {}", msg)),
            InfrastructureError::JwtError(msg) => ApplicationError::AuthenticationError(format!("JWT error: {}", msg)),
            InfrastructureError::PasswordError(msg) => ApplicationError::AuthenticationError(format!("Password error: {}", msg)),
            InfrastructureError::SerializationError(msg) => ApplicationError::UnexpectedError(format!("Serialization error: {}", msg)),
            InfrastructureError::ApplicationError(err) => err,
        }
    }
}
