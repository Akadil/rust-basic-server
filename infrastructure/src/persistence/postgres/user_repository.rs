use async_trait::async_trait;
use domain::entities::role::{Role, RoleName};
use domain::entities::User;
use domain::errors::DomainError;
use domain::repositories::UserRepository;
use sqlx::{PgPool, Row};
use std::collections::HashSet;
use std::str::FromStr;
use tracing::{info, instrument};
use uuid::Uuid;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    #[instrument(skip(self, user), fields(user_id = %user.id, username = %user.username))]
    async fn create(&self, user: &User) -> Result<(), DomainError> {
        info!("Creating user in PostgreSQL repository");
        
        let role_name = serde_json::to_value(&user.role.name)
            .map_err(|e| DomainError::RepositoryError(format!("Serialization error: {}", e)))?;
            
        let permissions = serde_json::to_value(&user.role.permissions)
            .map_err(|e| DomainError::RepositoryError(format!("Serialization error: {}", e)))?;

        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, role_name, role_permissions, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(role_name)
        .bind(permissions)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(format!("Database error: {}", e)))?;

        Ok(())
    }

    #[instrument(skip(self, user), fields(user_id = %user.id, username = %user.username))]
    async fn update(&self, user: &User) -> Result<(), DomainError> {
        info!("Updating user in PostgreSQL repository");
        
        let role_name = serde_json::to_value(&user.role.name)
            .map_err(|e| DomainError::RepositoryError(format!("Serialization error: {}", e)))?;
            
        let permissions = serde_json::to_value(&user.role.permissions)
            .map_err(|e| DomainError::RepositoryError(format!("Serialization error: {}", e)))?;

        let result = sqlx::query(
            r#"
            UPDATE users
            SET username = $1, email = $2, password_hash = $3, 
                role_name = $4, role_permissions = $5, updated_at = $6
            WHERE id = $7
            "#,
        )
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(role_name)
        .bind(permissions)
        .bind(user.updated_at)
        .bind(user.id)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(format!("Database error: {}", e)))?;

        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound(format!("User with ID {} not found", user.id)));
        }

        Ok(())
    }

    #[instrument(skip(self), fields(user_id = %id))]
    async fn delete(&self, id: &Uuid) -> Result<(), DomainError> {
        info!("Deleting user in PostgreSQL repository");
        
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::RepositoryError(format!("Database error: {}", e)))?;

        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound(format!("User with ID {} not found", id)));
        }

        Ok(())
    }

    #[instrument(skip(self), fields(user_id = %id))]
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, DomainError> {
        info!("Finding user by ID in PostgreSQL repository");
        
        let user = sqlx::query(
            r#"
            SELECT id, username, email, password_hash, role_name, role_permissions, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(format!("Database error: {}", e)))?;

        match user {
            Some(row) => {
                let role_name: String = row.get("role_name");
                let role_name = serde_json::from_str::<RoleName>(&role_name)
                    .map_err(|e| DomainError::RepositoryError(format!("Deserialization error: {}", e)))?;

                let permissions: String = row.get("role_permissions");
                let permissions = serde_json::from_str::<HashSet<String>>(&permissions)
                    .map_err(|e| DomainError::RepositoryError(format!("Deserialization error: {}", e)))?;

                let role = Role {
                    name: role_name,
                    permissions,
                };

                Ok(Some(User {
                    id: row.get("id"),
                    username: row.get("username"),
                    email: row.get("email"),
                    password_hash: row.get("password_hash"),
                    role,
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }))
            }
            None => Ok(None),
        }
    }

    #[instrument(skip(self), fields(username = %username))]
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError> {
        info!("Finding user by username in PostgreSQL repository");
        
        let user = sqlx::query(
            r#"
            SELECT id, username, email, password_hash, role_name, role_permissions, created_at, updated_at
            FROM users
            WHERE username = $1
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(format!("Database error: {}", e)))?;

        match user {
            Some(row) => {
                let role_name: String = row.get("role_name");
                let role_name = serde_json::from_str::<RoleName>(&role_name)
                    .map_err(|e| DomainError::RepositoryError(format!("Deserialization error: {}", e)))?;

                let permissions: String = row.get("role_permissions");
                let permissions = serde_json::from_str::<HashSet<String>>(&permissions)
                    .map_err(|e| DomainError::RepositoryError(format!("Deserialization error: {}", e)))?;

                let role = Role {
                    name: role_name,
                    permissions,
                };

                Ok(Some(User {
                    id: row.get("id"),
                    username: row.get("username"),
                    email: row.get("email"),
                    password_hash: row.get("password_hash"),
                    role,
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }))
            }
            None => Ok(None),
        }
    }

    #[instrument(skip(self), fields(email = %email))]
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        info!("Finding user by email in PostgreSQL repository");
        
        let user = sqlx::query(
            r#"
            SELECT id, username, email, password_hash, role_name, role_permissions, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(format!("Database error: {}", e)))?;

        match user {
            Some(row) => {
                let role_name: String = row.get("role_name");
                let role_name = serde_json::from_str::<RoleName>(&role_name)
                    .map_err(|e| DomainError::RepositoryError(format!("Deserialization error: {}", e)))?;

                let permissions: String = row.get("role_permissions");
                let permissions = serde_json::from_str::<HashSet<String>>(&permissions)
                    .map_err(|e| DomainError::RepositoryError(format!("Deserialization error: {}", e)))?;

                let role = Role {
                    name: role_name,
                    permissions,
                };

                Ok(Some(User {
                    id: row.get("id"),
                    username: row.get("username"),
                    email: row.get("email"),
                    password_hash: row.get("password_hash"),
                    role,
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }))
            }
            None => Ok(None),
        }
    }

    #[instrument(skip(self))]
    async fn find_all(&self) -> Result<Vec<User>, DomainError> {
        info!("Finding all users in PostgreSQL repository");
        
        let rows = sqlx::query(
            r#"
            SELECT id, username, email, password_hash, role_name, role_permissions, created_at, updated_at
            FROM users
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(format!("Database error: {}", e)))?;

        let mut users = Vec::with_capacity(rows.len());

        for row in rows {
            let role_name: String = row.get("role_name");
            let role_name = serde_json::from_str::<RoleName>(&role_name)
                .map_err(|e| DomainError::RepositoryError(format!("Deserialization error: {}", e)))?;

            let permissions: String = row.get("role_permissions");
            let permissions = serde_json::from_str::<HashSet<String>>(&permissions)
                .map_err(|e| DomainError::RepositoryError(format!("Deserialization error: {}", e)))?;

            let role = Role {
                name: role_name,
                permissions,
            };

            users.push(User {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
                password_hash: row.get("password_hash"),
                role,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(users)
    }
}
