use domain::errors::DomainError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),
    
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    
    #[error("Authorization error: {0}")]
    AuthorizationError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}
