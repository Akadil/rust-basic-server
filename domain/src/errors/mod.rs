use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Entity not found: {0}")]
    NotFound(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    
    #[error("Authorization error: {0}")]
    AuthorizationError(String),
    
    #[error("Repository error: {0}")]
    RepositoryError(String),
    
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}
