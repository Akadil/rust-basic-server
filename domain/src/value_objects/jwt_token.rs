use crate::entities::role::RoleName;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtToken {
    pub token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,  // User ID
    pub exp: i64,     // Expiration time (as UTC timestamp)
    pub iat: i64,     // Issued at (as UTC timestamp)
    pub role: RoleName,
}

impl JwtToken {
    pub fn new(token: String, expires_at: chrono::DateTime<chrono::Utc>) -> Self {
        Self { token, expires_at }
    }

    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() > self.expires_at
    }
}

impl JwtClaims {
    pub fn new(user_id: Uuid, role: RoleName, expires_in_seconds: i64) -> Self {
        let now = chrono::Utc::now();
        let iat = now.timestamp();
        let exp = now.timestamp() + expires_in_seconds;

        Self {
            sub: user_id.to_string(),
            exp,
            iat,
            role,
        }
    }
}
