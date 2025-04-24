use application::errors::ApplicationError;
use application::services::PasswordService;
use async_trait::async_trait;
use tracing::{info, instrument};

use crate::errors::InfrastructureError;

pub struct BcryptPasswordService {
    cost: u32,
}

impl BcryptPasswordService {
    pub fn new(cost: Option<u32>) -> Self {
        Self {
            cost: cost.unwrap_or(12), // Default cost
        }
    }
}

#[async_trait]
impl PasswordService for BcryptPasswordService {
    #[instrument(skip(self, password))]
    fn hash_password(&self, password: &str) -> Result<String, ApplicationError> {
        info!("Hashing password");
        
        bcrypt::hash(password, self.cost).map_err(|e| {
            let err = InfrastructureError::PasswordError(format!("Failed to hash password: {}", e));
            ApplicationError::from(err)
        })
    }

    #[instrument(skip(self, password, hash))]
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, ApplicationError> {
        info!("Verifying password");
        
        bcrypt::verify(password, hash).map_err(|e| {
            let err = InfrastructureError::PasswordError(format!("Failed to verify password: {}", e));
            ApplicationError::from(err)
        })
    }
}
