use crate::dtos::{LoginRequestDto, LoginResponseDto, RegisterRequestDto, RegisterResponseDto};
use crate::errors::ApplicationError;
use async_trait::async_trait;
use domain::entities::role::{Role, RoleName};
use domain::entities::User;
use domain::repositories::UserRepository;
use domain::value_objects::JwtClaims;
use std::sync::Arc;
use tracing::{info, instrument};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn login(&self, request: LoginRequestDto) -> Result<LoginResponseDto, ApplicationError>;
    async fn register(&self, request: RegisterRequestDto) -> Result<RegisterResponseDto, ApplicationError>;
    async fn validate_token(&self, token: &str) -> Result<JwtClaims, ApplicationError>;
}

pub struct AuthServiceImpl<T: UserRepository> {
    user_repository: Arc<T>,
    jwt_service: Arc<dyn JwtService>,
    password_service: Arc<dyn PasswordService>,
}

#[async_trait]
pub trait JwtService: Send + Sync {
    fn generate_token(&self, claims: JwtClaims) -> Result<String, ApplicationError>;
    fn validate_token(&self, token: &str) -> Result<JwtClaims, ApplicationError>;
}

#[async_trait]
pub trait PasswordService: Send + Sync {
    fn hash_password(&self, password: &str) -> Result<String, ApplicationError>;
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, ApplicationError>;
}

impl<T: UserRepository> AuthServiceImpl<T> {
    pub fn new(
        user_repository: Arc<T>,
        jwt_service: Arc<dyn JwtService>,
        password_service: Arc<dyn PasswordService>,
    ) -> Self {
        Self {
            user_repository,
            jwt_service,
            password_service,
        }
    }
}

#[async_trait]
impl<T: UserRepository> AuthService for AuthServiceImpl<T> {
    #[instrument(skip(self, request), fields(username = %request.username))]
    async fn login(&self, request: LoginRequestDto) -> Result<LoginResponseDto, ApplicationError> {
        info!("Attempting login for user: {}", request.username);
        
        let user = self
            .user_repository
            .find_by_username(&request.username)
            .await?
            .ok_or_else(|| ApplicationError::AuthenticationError("Invalid username or password".to_string()))?;

        let password_valid = self
            .password_service
            .verify_password(&request.password, &user.password_hash)?;

        if !password_valid {
            return Err(ApplicationError::AuthenticationError(
                "Invalid username or password".to_string(),
            ));
        }

        let claims = JwtClaims::new(user.id, user.role.name.clone(), 3600); // 1 hour token
        let token = self.jwt_service.generate_token(claims)?;

        let expires_at = chrono::Utc::now() + chrono::Duration::seconds(3600);

        info!("Login successful for user: {}", request.username);
        
        Ok(LoginResponseDto {
            token,
            expires_at,
            user_id: user.id.to_string(),
            username: user.username,
            role: user.role.name,
        })
    }

    #[instrument(skip(self, request), fields(username = %request.username, email = %request.email))]
    async fn register(&self, request: RegisterRequestDto) -> Result<RegisterResponseDto, ApplicationError> {
        info!("Attempting registration for user: {}", request.username);
        
        // Check if username already exists
        if let Some(_) = self.user_repository.find_by_username(&request.username).await? {
            return Err(ApplicationError::ValidationError(
                "Username already exists".to_string(),
            ));
        }

        // Check if email already exists
        if let Some(_) = self.user_repository.find_by_email(&request.email).await? {
            return Err(ApplicationError::ValidationError(
                "Email already exists".to_string(),
            ));
        }

        let password_hash = self.password_service.hash_password(&request.password)?;
        
        // Default to User role if not specified
        let role = match request.role {
            Some(role_name) => Role::new(role_name),
            None => Role::new(RoleName::User),
        };

        let user = User::new(request.username.clone(), request.email.clone(), password_hash, role.clone());

        self.user_repository.create(&user).await?;

        info!("Registration successful for user: {}", request.username);
        
        Ok(RegisterResponseDto {
            user_id: user.id.to_string(),
            username: user.username,
            email: user.email,
            role: role.name,
        })
    }

    #[instrument(skip(self, token))]
    async fn validate_token(&self, token: &str) -> Result<JwtClaims, ApplicationError> {
        info!("Validating JWT token");
        self.jwt_service.validate_token(token)
    }
}
