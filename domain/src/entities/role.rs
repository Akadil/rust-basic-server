use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Role {
    pub name: RoleName,
    pub permissions: HashSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RoleName {
    Admin,
    Manager,
    User,
    Guest,
}

impl Role {
    pub fn new(name: RoleName) -> Self {
        let permissions = match name {
            RoleName::Admin => {
                let mut perms = HashSet::new();
                perms.insert("users:read".to_string());
                perms.insert("users:write".to_string());
                perms.insert("users:delete".to_string());
                perms.insert("roles:read".to_string());
                perms.insert("roles:write".to_string());
                perms.insert("roles:delete".to_string());
                perms
            }
            RoleName::Manager => {
                let mut perms = HashSet::new();
                perms.insert("users:read".to_string());
                perms.insert("users:write".to_string());
                perms.insert("roles:read".to_string());
                perms
            }
            RoleName::User => {
                let mut perms = HashSet::new();
                perms.insert("users:read".to_string());
                perms
            }
            RoleName::Guest => HashSet::new(),
        };

        Self { name, permissions }
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(permission)
    }
}
