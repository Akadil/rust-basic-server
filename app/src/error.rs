use application::errors::ApplicationError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Authorization error: {0}")]
    AuthorizationError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl From<ApplicationError> for ApiError {
    fn from(error: ApplicationError) -> Self {
        match error {
            ApplicationError::AuthenticationError(msg) => ApiError::AuthenticationError(msg),
            ApplicationError::AuthorizationError(msg) => ApiError::AuthorizationError(msg),
            ApplicationError::ValidationError(msg) => ApiError::ValidationError(msg),
            ApplicationError::NotFound(msg) => ApiError::NotFound(msg),
            ApplicationError::DomainError(domain_error) => {
                ApiError::InternalServerError(format!("Domain error: {}", domain_error))
            }
            ApplicationError::UnexpectedError(msg) => ApiError::InternalServerError(msg),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::AuthenticationError(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::AuthorizationError(msg) => (StatusCode::FORBIDDEN, msg),
            ApiError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "error": {
                "message": error_message,
                "status": status.as_u16()
            }
        }));

        (status, body).into_response()
    }
}
