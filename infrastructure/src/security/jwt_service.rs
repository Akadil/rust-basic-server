use application::errors::ApplicationError;
use application::services::JwtService;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use domain::value_objects::{JwtClaims, JwtToken};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::sync::Arc;
use tracing::{info, instrument};

use crate::config::ConfigProvider;
use crate::errors::InfrastructureError;

pub struct JwtServiceImpl {
    config_provider: Arc<dyn ConfigProvider>,
}

impl JwtServiceImpl {
    pub fn new(config_provider: Arc<dyn ConfigProvider>) -> Self {
        Self { config_provider }
    }
}

#[async_trait]
impl JwtService for JwtServiceImpl {
    #[instrument(skip(self, claims))]
    fn generate_token(&self, claims: JwtClaims) -> Result<String, ApplicationError> {
        info!("Generating JWT token");
        
        let config = self.config_provider.get_config();
        let secret = config.jwt.secret.as_bytes();
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret),
        )
        .map_err(|e| {
            let err = InfrastructureError::JwtError(format!("Failed to generate token: {}", e));
            ApplicationError::from(err)
        })?;
        
        Ok(token)
    }

    #[instrument(skip(self, token))]
    fn validate_token(&self, token: &str) -> Result<JwtClaims, ApplicationError> {
        info!("Validating JWT token");
        
        let config = self.config_provider.get_config();
        let secret = config.jwt.secret.as_bytes();
        
        let token_data = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(secret),
            &Validation::default(),
        )
        .map_err(|e| {
            let err = InfrastructureError::JwtError(format!("Invalid token: {}", e));
            ApplicationError::from(err)
        })?;
        
        let claims = token_data.claims;
        
        // Check if token is expired
        let now = Utc::now().timestamp();
        if claims.exp < now {
            return Err(ApplicationError::AuthenticationError("Token expired".to_string()));
        }
        
        Ok(claims)
    }
}

pub fn create_token(
    user_id: &str,
    role: domain::entities::role::RoleName,
    jwt_service: &dyn JwtService,
    expiration_seconds: i64,
) -> Result<JwtToken, ApplicationError> {
    let user_id = uuid::Uuid::parse_str(user_id).map_err(|_| {
        ApplicationError::ValidationError("Invalid user ID format".to_string())
    })?;
    
    let claims = JwtClaims::new(user_id, role, expiration_seconds);
    let token = jwt_service.generate_token(claims)?;
    
    let expires_at = Utc::now() + Duration::seconds(expiration_seconds);
    
    Ok(JwtToken::new(token, expires_at))
}
