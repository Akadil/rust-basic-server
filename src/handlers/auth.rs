use crate::{
    error::{AppError, Result},
    models::{CreateUserDto, LoginUserDto, User, UserResponse},
    utils::generate_token,
};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;
use std::sync::RwLock;
use once_cell::sync::Lazy;

// In-memory user storage (for simplicity)
static USERS: Lazy<RwLock<Vec<User>>> = Lazy::new(|| RwLock::new(Vec::new()));

pub async fn register(
    Json(payload): Json<CreateUserDto>,
) -> Result<impl IntoResponse> {
    // Check if user with the same email already exists
    {
        let users = USERS.read().unwrap();
        if users.iter().any(|u| u.email == payload.email) {
            return Err(AppError::Validation("Email already exists".to_string()));
        }
    }

    // Hash the password
    let hashed_password = hash(payload.password, DEFAULT_COST)?;

    // Create a new user
    let now = Utc::now();
    let user = User {
        id: Uuid::new_v4(),
        username: payload.username,
        email: payload.email,
        password: hashed_password,
        created_at: now,
        updated_at: now,
    };

    // Generate JWT token
    let token = generate_token(&user)?;

    // Store the user
    {
        let mut users = USERS.write().unwrap();
        users.push(user.clone());
    }

    // Return user data and token
    let user_response = UserResponse::from(user);
    
    Ok((
        StatusCode::CREATED,
        Json(json!({
            "status": "success",
            "message": "User registered successfully",
            "user": user_response,
            "token": token
        })),
    ))
}

pub async fn login(
    Json(payload): Json<LoginUserDto>,
) -> Result<impl IntoResponse> {
    // Find user by email
    let users = USERS.read().unwrap();
    let user = users
        .iter()
        .find(|u| u.email == payload.email)
        .ok_or_else(|| AppError::Auth("Invalid email or password".to_string()))?;

    // Verify password
    let is_valid = verify(payload.password, &user.password)
        .map_err(|_| AppError::Auth("Invalid email or password".to_string()))?;

    if !is_valid {
        return Err(AppError::Auth("Invalid email or password".to_string()));
    }

    // Generate JWT token
    let token = generate_token(user)?;

    // Return user data and token
    let user_response = UserResponse::from(user.clone());
    
    Ok((
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "User logged in successfully",
            "user": user_response,
            "token": token
        })),
    ))
}
