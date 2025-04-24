use crate::entities::User;
use crate::errors::DomainError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<(), DomainError>;
    async fn update(&self, user: &User) -> Result<(), DomainError>;
    async fn delete(&self, id: &Uuid) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, DomainError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn find_all(&self) -> Result<Vec<User>, DomainError>;
}
