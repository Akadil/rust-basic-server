use crate::api::AppState;
use crate::error::ApiError;
use application::dtos::{CreateUserDto, UpdateUserDto, UserDto};
use axum::{
    extract::{Path, State},
    Json,
};
use domain::entities::role::RoleName;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: RoleName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<RoleName>,
}

/// Get user by ID
///
/// Get a user by their ID. Requires authentication.
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<UserDto>, ApiError> {
    info!("Get user request received for ID: {}", id);
    
    let user = state.user_use_cases.get_user(&id).await?;
    
    Ok(Json(user))
}

/// Get all users
///
/// Get a list of all users. Requires authentication.
pub async fn get_all_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserDto>>, ApiError> {
    info!("Get all users request received");
    
    let users = state.user_use_cases.get_all_users().await?;
    
    Ok(Json(users))
}

/// Create user
///
/// Create a new user. Requires authentication and admin role.
pub async fn create_user(
    State(state): State<AppState>,
    Json(user_request): Json<CreateUserRequest>,
) -> Result<Json<UserDto>, ApiError> {
    info!("Create user request received for: {}", user_request.username);
    
    let create_user_dto = CreateUserDto {
        username: user_request.username,
        email: user_request.email,
        password: user_request.password,
        role: user_request.role,
    };
    
    let user = state.user_use_cases.create_user(create_user_dto).await?;
    
    Ok(Json(user))
}

/// Update user
///
/// Update an existing user. Requires authentication and appropriate role.
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(user_request): Json<UpdateUserRequest>,
) -> Result<Json<UserDto>, ApiError> {
    info!("Update user request received for ID: {}", id);
    
    let update_user_dto = UpdateUserDto {
        username: user_request.username,
        email: user_request.email,
        password: user_request.password,
        role: user_request.role,
    };
    
    let user = state.user_use_cases.update_user(&id, update_user_dto).await?;
    
    Ok(Json(user))
}

/// Delete user
///
/// Delete a user by their ID. Requires authentication and admin role.
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<(), ApiError> {
    info!("Delete user request received for ID: {}", id);
    
    state.user_use_cases.delete_user(&id).await?;
    
    Ok(())
}
