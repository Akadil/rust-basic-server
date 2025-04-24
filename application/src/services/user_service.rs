use crate::dtos::{CreateUserDto, UpdateUserDto, UserDto};
use crate::errors::ApplicationError;
use async_trait::async_trait;
use domain::entities::role::Role;
use domain::entities::User;
use domain::repositories::UserRepository;
use std::sync::Arc;
use tracing::{info, instrument};
use uuid::Uuid;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_user_by_id(&self, id: &str) -> Result<UserDto, ApplicationError>;
    async fn get_all_users(&self) -> Result<Vec<UserDto>, ApplicationError>;
    async fn create_user(&self, user: CreateUserDto) -> Result<UserDto, ApplicationError>;
    async fn update_user(&self, id: &str, user: UpdateUserDto) -> Result<UserDto, ApplicationError>;
    async fn delete_user(&self, id: &str) -> Result<(), ApplicationError>;
}

pub struct UserServiceImpl<T: UserRepository> {
    user_repository: Arc<T>,
    password_service: Arc<dyn super::auth_service::PasswordService>,
}

impl<T: UserRepository> UserServiceImpl<T> {
    pub fn new(
        user_repository: Arc<T>,
        password_service: Arc<dyn super::auth_service::PasswordService>,
    ) -> Self {
        Self {
            user_repository,
            password_service,
        }
    }

    fn map_to_dto(&self, user: User) -> UserDto {
        UserDto {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            role: user.role.name,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[async_trait]
impl<T: UserRepository> UserService for UserServiceImpl<T> {
    #[instrument(skip(self), fields(user_id = %id))]
    async fn get_user_by_id(&self, id: &str) -> Result<UserDto, ApplicationError> {
        info!("Getting user by ID: {}", id);
        
        let uuid = Uuid::parse_str(id).map_err(|_| {
            ApplicationError::ValidationError("Invalid user ID format".to_string())
        })?;

        let user = self
            .user_repository
            .find_by_id(&uuid)
            .await?
            .ok_or_else(|| ApplicationError::NotFound(format!("User with ID {} not found", id)))?;

        Ok(self.map_to_dto(user))
    }

    #[instrument(skip(self))]
    async fn get_all_users(&self) -> Result<Vec<UserDto>, ApplicationError> {
        info!("Getting all users");
        
        let users = self.user_repository.find_all().await?;
        let user_dtos = users.into_iter().map(|user| self.map_to_dto(user)).collect();

        Ok(user_dtos)
    }

    #[instrument(skip(self, user), fields(username = %user.username, email = %user.email))]
    async fn create_user(&self, user: CreateUserDto) -> Result<UserDto, ApplicationError> {
        info!("Creating new user: {}", user.username);
        
        // Check if username already exists
        if let Some(_) = self.user_repository.find_by_username(&user.username).await? {
            return Err(ApplicationError::ValidationError(
                "Username already exists".to_string(),
            ));
        }

        // Check if email already exists
        if let Some(_) = self.user_repository.find_by_email(&user.email).await? {
            return Err(ApplicationError::ValidationError(
                "Email already exists".to_string(),
            ));
        }

        let password_hash = self.password_service.hash_password(&user.password)?;
        let role = Role::new(user.role);
        let new_user = User::new(user.username, user.email, password_hash, role);

        self.user_repository.create(&new_user).await?;

        Ok(self.map_to_dto(new_user))
    }

    #[instrument(skip(self, user), fields(user_id = %id))]
    async fn update_user(&self, id: &str, user: UpdateUserDto) -> Result<UserDto, ApplicationError> {
        info!("Updating user with ID: {}", id);
        
        let uuid = Uuid::parse_str(id).map_err(|_| {
            ApplicationError::ValidationError("Invalid user ID format".to_string())
        })?;

        let mut existing_user = self
            .user_repository
            .find_by_id(&uuid)
            .await?
            .ok_or_else(|| ApplicationError::NotFound(format!("User with ID {} not found", id)))?;

        // Update fields if provided
        if let Some(username) = user.username {
            // Check if the new username is already taken by another user
            if let Some(found_user) = self.user_repository.find_by_username(&username).await? {
                if found_user.id != uuid {
                    return Err(ApplicationError::ValidationError(
                        "Username already exists".to_string(),
                    ));
                }
            }
            existing_user.username = username;
        }

        if let Some(email) = user.email {
            // Check if the new email is already taken by another user
            if let Some(found_user) = self.user_repository.find_by_email(&email).await? {
                if found_user.id != uuid {
                    return Err(ApplicationError::ValidationError(
                        "Email already exists".to_string(),
                    ));
                }
            }
            existing_user.email = email;
        }

        if let Some(password) = user.password {
            existing_user.password_hash = self.password_service.hash_password(&password)?;
        }

        if let Some(role_name) = user.role {
            existing_user.role = Role::new(role_name);
        }

        existing_user.updated_at = chrono::Utc::now();

        self.user_repository.update(&existing_user).await?;

        Ok(self.map_to_dto(existing_user))
    }

    #[instrument(skip(self), fields(user_id = %id))]
    async fn delete_user(&self, id: &str) -> Result<(), ApplicationError> {
        info!("Deleting user with ID: {}", id);
        
        let uuid = Uuid::parse_str(id).map_err(|_| {
            ApplicationError::ValidationError("Invalid user ID format".to_string())
        })?;

        // Check if user exists
        let _ = self
            .user_repository
            .find_by_id(&uuid)
            .await?
            .ok_or_else(|| ApplicationError::NotFound(format!("User with ID {} not found", id)))?;

        self.user_repository.delete(&uuid).await?;

        Ok(())
    }
}
