use domain::entities::role::RoleName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequestDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponseDto {
    pub token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
    pub username: String,
    pub role: RoleName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequestDto {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Option<RoleName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterResponseDto {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub role: RoleName,
}
