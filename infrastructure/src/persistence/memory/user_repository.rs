use async_trait::async_trait;
use domain::entities::User;
use domain::errors::DomainError;
use domain::repositories::UserRepository;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{info, instrument};
use uuid::Uuid;

pub struct InMemoryUserRepository {
    users: Arc<RwLock<HashMap<Uuid, User>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    #[instrument(skip(self, user), fields(user_id = %user.id, username = %user.username))]
    async fn create(&self, user: &User) -> Result<(), DomainError> {
        info!("Creating user in in-memory repository");
        
        let mut users = self.users.write().map_err(|e| {
            DomainError::RepositoryError(format!("Failed to acquire write lock: {}", e))
        })?;

        // Check if username already exists
        for existing_user in users.values() {
            if existing_user.username == user.username {
                return Err(DomainError::ValidationError(format!(
                    "Username {} already exists",
                    user.username
                )));
            }
            if existing_user.email == user.email {
                return Err(DomainError::ValidationError(format!(
                    "Email {} already exists",
                    user.email
                )));
            }
        }

        users.insert(user.id, user.clone());
        Ok(())
    }

    #[instrument(skip(self, user), fields(user_id = %user.id, username = %user.username))]
    async fn update(&self, user: &User) -> Result<(), DomainError> {
        info!("Updating user in in-memory repository");
        
        let mut users = self.users.write().map_err(|e| {
            DomainError::RepositoryError(format!("Failed to acquire write lock: {}", e))
        })?;

        if !users.contains_key(&user.id) {
            return Err(DomainError::NotFound(format!("User with ID {} not found", user.id)));
        }

        // Check if username already exists for another user
        for (id, existing_user) in users.iter() {
            if *id != user.id {
                if existing_user.username == user.username {
                    return Err(DomainError::ValidationError(format!(
                        "Username {} already exists",
                        user.username
                    )));
                }
                if existing_user.email == user.email {
                    return Err(DomainError::ValidationError(format!(
                        "Email {} already exists",
                        user.email
                    )));
                }
            }
        }

        users.insert(user.id, user.clone());
        Ok(())
    }

    #[instrument(skip(self), fields(user_id = %id))]
    async fn delete(&self, id: &Uuid) -> Result<(), DomainError> {
        info!("Deleting user in in-memory repository");
        
        let mut users = self.users.write().map_err(|e| {
            DomainError::RepositoryError(format!("Failed to acquire write lock: {}", e))
        })?;

        if users.remove(id).is_none() {
            return Err(DomainError::NotFound(format!("User with ID {} not found", id)));
        }

        Ok(())
    }

    #[instrument(skip(self), fields(user_id = %id))]
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, DomainError> {
        info!("Finding user by ID in in-memory repository");
        
        let users = self.users.read().map_err(|e| {
            DomainError::RepositoryError(format!("Failed to acquire read lock: {}", e))
        })?;

        Ok(users.get(id).cloned())
    }

    #[instrument(skip(self), fields(username = %username))]
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError> {
        info!("Finding user by username in in-memory repository");
        
        let users = self.users.read().map_err(|e| {
            DomainError::RepositoryError(format!("Failed to acquire read lock: {}", e))
        })?;

        Ok(users
            .values()
            .find(|user| user.username == username)
            .cloned())
    }

    #[instrument(skip(self), fields(email = %email))]
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        info!("Finding user by email in in-memory repository");
        
        let users = self.users.read().map_err(|e| {
            DomainError::RepositoryError(format!("Failed to acquire read lock: {}", e))
        })?;

        Ok(users
            .values()
            .find(|user| user.email == email)
            .cloned())
    }

    #[instrument(skip(self))]
    async fn find_all(&self) -> Result<Vec<User>, DomainError> {
        info!("Finding all users in in-memory repository");
        
        let users = self.users.read().map_err(|e| {
            DomainError::RepositoryError(format!("Failed to acquire read lock: {}", e))
        })?;

        Ok(users.values().cloned().collect())
    }
}

impl Default for InMemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}
