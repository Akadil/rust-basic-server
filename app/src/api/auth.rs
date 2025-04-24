use crate::api::AppState;
use crate::error::ApiError;
use application::dtos::{LoginRequestDto, LoginResponseDto, RegisterRequestDto, RegisterResponseDto};
use axum::{extract::State, Json};
use tracing::{info, instrument};

/// Login user
///
/// Login with username and password to get a JWT token.
pub async fn login(
    State(state): State<AppState>,
    Json(login_request): Json<LoginRequestDto>,
) -> Result<Json<LoginResponseDto>, ApiError> {
    info!("Login request received for user: {}", login_request.username);
    
    let response = state.auth_use_cases.login(login_request).await?;
    
    Ok(Json(response))
}

/// Register new user
///
/// Register a new user with username, email, and password.
pub async fn register(
    State(state): State<AppState>,
    Json(register_request): Json<RegisterRequestDto>,
) -> Result<Json<RegisterResponseDto>, ApiError> {
    info!("Registration request received for user: {}", register_request.username);
    
    let response = state.auth_use_cases.register(register_request).await?;
    
    Ok(Json(response))
}
