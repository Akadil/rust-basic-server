use crate::error::ApiError;
use application::services::JwtService;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use domain::entities::role::RoleName;
use std::sync::Arc;
use tracing::{info, instrument};

#[derive(Clone)]
pub struct AuthState {
    pub jwt_service: Arc<dyn JwtService>,
}

#[instrument(skip(state, request, next))]
pub async fn auth_middleware<B>(
    State(state): State<AuthState>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, ApiError> {
    info!("Running auth middleware");

    // Extract the token from the Authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| ApiError::AuthenticationError("Missing Authorization header".to_string()))?;

    // Check if the header starts with "Bearer "
    if !auth_header.starts_with("Bearer ") {
        return Err(ApiError::AuthenticationError(
            "Invalid Authorization header format".to_string(),
        ));
    }

    // Extract the token
    let token = &auth_header[7..];

    // Validate the token
    let claims = state
        .jwt_service
        .validate_token(token)
        .await
        .map_err(|e| ApiError::AuthenticationError(format!("Invalid token: {}", e)))?;

    // Add the user ID and role to the request extensions
    request.extensions_mut().insert(claims.sub.clone());
    request.extensions_mut().insert(claims.role);

    // Continue with the request
    Ok(next.run(request).await)
}

#[instrument(skip(request, next))]
pub async fn require_role<B>(
    role: RoleName,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, ApiError> {
    info!("Checking role requirement: {:?}", role);

    // Get the user's role from the request extensions
    let user_role = request
        .extensions()
        .get::<RoleName>()
        .ok_or_else(|| ApiError::AuthorizationError("User role not found".to_string()))?;

    // Check if the user has the required role
    match (user_role, &role) {
        (RoleName::Admin, _) => {
            // Admins can access everything
            Ok(next.run(request).await)
        }
        (RoleName::Manager, RoleName::Admin) => {
            // Managers cannot access admin routes
            Err(ApiError::AuthorizationError(
                "Insufficient permissions".to_string(),
            ))
        }
        (RoleName::Manager, _) => {
            // Managers can access manager and below routes
            Ok(next.run(request).await)
        }
        (RoleName::User, RoleName::Admin | RoleName::Manager) => {
            // Users cannot access admin or manager routes
            Err(ApiError::AuthorizationError(
                "Insufficient permissions".to_string(),
            ))
        }
        (RoleName::User, _) => {
            // Users can access user and below routes
            Ok(next.run(request).await)
        }
        (RoleName::Guest, RoleName::Admin | RoleName::Manager | RoleName::User) => {
            // Guests cannot access admin, manager, or user routes
            Err(ApiError::AuthorizationError(
                "Insufficient permissions".to_string(),
            ))
        }
        (RoleName::Guest, _) => {
            // Guests can access guest routes
            Ok(next.run(request).await)
        }
    }
}
