use crate::{
    config::CONFIG,
    error::{AppError, Result},
    models::User,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,    // Subject (user ID)
    pub exp: usize,     // Expiration time (as UTC timestamp)
    pub iat: usize,     // Issued at (as UTC timestamp)
}

pub fn generate_token(user: &User) -> Result<String> {
    let now = Utc::now();
    let expires_at = now + Duration::seconds(CONFIG.jwt_expires_in as i64);
    
    let claims = Claims {
        sub: user.id.to_string(),
        exp: expires_at.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &CONFIG.encoding_key(),
    )
    .map_err(AppError::Jwt)
}

pub fn verify_token(token: &str) -> Result<Claims> {
    let validation = Validation::new(Algorithm::HS256);
    
    let token_data = decode::<Claims>(
        token,
        &CONFIG.decoding_key(),
        &validation,
    )
    .map_err(AppError::Jwt)?;
    
    Ok(token_data.claims)
}

pub fn get_user_id_from_token(token: &str) -> Result<Uuid> {
    let claims = verify_token(token)?;
    
    Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Auth("Invalid user ID in token".to_string()))
}
