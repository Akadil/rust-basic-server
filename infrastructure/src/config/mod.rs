use serde::Deserialize;
use std::env;
use std::sync::Arc;
use tracing::info;

use crate::errors::InfrastructureError;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub cors: CorsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: i64, // in seconds
}

#[derive(Debug, Clone, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allow_credentials: bool,
}

pub trait ConfigProvider: Send + Sync {
    fn get_config(&self) -> &AppConfig;
}

pub struct EnvConfigProvider {
    config: AppConfig,
}

impl EnvConfigProvider {
    pub fn new() -> Result<Self, InfrastructureError> {
        // Load .env file if it exists
        dotenv::dotenv().ok();

        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .map_err(|e| InfrastructureError::ConfigurationError(format!("Invalid port: {}", e)))?;

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@localhost:5432/rust_server".to_string()
        });
        let database_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<u32>()
            .map_err(|e| {
                InfrastructureError::ConfigurationError(format!("Invalid max connections: {}", e))
            })?;

        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "super_secret_key".to_string());
        let jwt_expiration = env::var("JWT_EXPIRATION")
            .unwrap_or_else(|_| "3600".to_string()) // 1 hour default
            .parse::<i64>()
            .map_err(|e| {
                InfrastructureError::ConfigurationError(format!("Invalid JWT expiration: {}", e))
            })?;

        let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let cors_allow_credentials = env::var("CORS_ALLOW_CREDENTIALS")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .map_err(|e| {
                InfrastructureError::ConfigurationError(format!(
                    "Invalid CORS allow credentials: {}",
                    e
                ))
            })?;

        let config = AppConfig {
            server: ServerConfig {
                host: server_host,
                port: server_port,
            },
            database: DatabaseConfig {
                url: database_url,
                max_connections: database_max_connections,
            },
            jwt: JwtConfig {
                secret: jwt_secret,
                expiration: jwt_expiration,
            },
            cors: CorsConfig {
                allowed_origins: cors_allowed_origins,
                allow_credentials: cors_allow_credentials,
            },
        };

        info!("Configuration loaded successfully");

        Ok(Self { config })
    }
}

impl ConfigProvider for EnvConfigProvider {
    fn get_config(&self) -> &AppConfig {
        &self.config
    }
}

pub type ConfigProviderType = Arc<dyn ConfigProvider>;
