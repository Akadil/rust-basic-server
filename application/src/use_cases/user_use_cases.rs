use crate::dtos::{CreateUserDto, UpdateUserDto, UserDto};
use crate::errors::ApplicationError;
use crate::services::UserService;
use std::sync::Arc;
use tracing::{info, instrument};

pub struct UserUseCases {
    user_service: Arc<dyn UserService>,
}

impl UserUseCases {
    pub fn new(user_service: Arc<dyn UserService>) -> Self {
        Self { user_service }
    }

    #[instrument(skip(self), fields(user_id = %id))]
    pub async fn get_user(&self, id: &str) -> Result<UserDto, ApplicationError> {
        info!("Get user use case for ID: {}", id);
        self.user_service.get_user_by_id(id).await
    }

    #[instrument(skip(self))]
    pub async fn get_all_users(&self) -> Result<Vec<UserDto>, ApplicationError> {
        info!("Get all users use case");
        self.user_service.get_all_users().await
    }

    #[instrument(skip(self, user), fields(username = %user.username, email = %user.email))]
    pub async fn create_user(&self, user: CreateUserDto) -> Result<UserDto, ApplicationError> {
        info!("Create user use case for: {}", user.username);
        self.user_service.create_user(user).await
    }

    #[instrument(skip(self, user), fields(user_id = %id))]
    pub async fn update_user(&self, id: &str, user: UpdateUserDto) -> Result<UserDto, ApplicationError> {
        info!("Update user use case for ID: {}", id);
        self.user_service.update_user(id, user).await
    }

    #[instrument(skip(self), fields(user_id = %id))]
    pub async fn delete_user(&self, id: &str) -> Result<(), ApplicationError> {
        info!("Delete user use case for ID: {}", id);
        self.user_service.delete_user(id).await
    }
}
