use crate::entities::role::Role;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: Role,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn new(username: String, email: String, password_hash: String, role: Role) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            password_hash,
            role,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.role.has_permission(permission)
    }
}
