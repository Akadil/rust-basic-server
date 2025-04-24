use crate::dtos::{LoginRequestDto, LoginResponseDto, RegisterRequestDto, RegisterResponseDto};
use crate::errors::ApplicationError;
use crate::services::AuthService;
use std::sync::Arc;
use tracing::{info, instrument};

pub struct AuthUseCases {
    auth_service: Arc<dyn AuthService>,
}

impl AuthUseCases {
    pub fn new(auth_service: Arc<dyn AuthService>) -> Self {
        Self { auth_service }
    }

    #[instrument(skip(self, request), fields(username = %request.username))]
    pub async fn login(&self, request: LoginRequestDto) -> Result<LoginResponseDto, ApplicationError> {
        info!("Login use case for user: {}", request.username);
        self.auth_service.login(request).await
    }

    #[instrument(skip(self, request), fields(username = %request.username, email = %request.email))]
    pub async fn register(&self, request: RegisterRequestDto) -> Result<RegisterResponseDto, ApplicationError> {
        info!("Register use case for user: {}", request.username);
        self.auth_service.register(request).await
    }

    #[instrument(skip(self, token))]
    pub async fn validate_token(&self, token: &str) -> Result<(), ApplicationError> {
        info!("Validate token use case");
        let _ = self.auth_service.validate_token(token).await?;
        Ok(())
    }
}
