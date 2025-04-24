mod user_repository;

pub use user_repository::*;

use crate::config::ConfigProvider;
use crate::errors::InfrastructureError;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::info;

pub async fn create_postgres_pool(
    config_provider: Arc<dyn ConfigProvider>,
) -> Result<PgPool, InfrastructureError> {
    let config = config_provider.get_config();
    let database_url = &config.database.url;
    let max_connections = config.database.max_connections;

    info!(
        "Creating PostgreSQL connection pool with max connections: {}",
        max_connections
    );

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await
        .map_err(|e| InfrastructureError::DatabaseError(format!("Database connection error: {}", e)))?;

    // Run migrations
    info!("Running database migrations");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| InfrastructureError::DatabaseError(format!("Migration error: {}", e)))?;

    info!("PostgreSQL connection pool created successfully");
    Ok(pool)
}
