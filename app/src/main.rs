mod api;
mod config;
mod error;
mod middleware;
#[cfg(test)]
mod tests;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use infrastructure::config::EnvConfigProvider;
use infrastructure::persistence::memory::InMemoryUserRepository;
use infrastructure::persistence::postgres::create_postgres_pool;
use infrastructure::security::{BcryptPasswordService, JwtServiceImpl};
use infrastructure::tracing::init_tracing;
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    init_tracing();

    info!("Starting server...");

    // Load configuration
    let config_provider = Arc::new(EnvConfigProvider::new()?);
    let config = config_provider.get_config();

    // Create PostgreSQL connection pool
    let pg_pool = create_postgres_pool(Arc::clone(&config_provider)).await?;

    // Create repositories
    let postgres_user_repo = Arc::new(infrastructure::persistence::postgres::PostgresUserRepository::new(pg_pool));
    let memory_user_repo = Arc::new(InMemoryUserRepository::new());

    // Create services
    let password_service = Arc::new(BcryptPasswordService::new(None));
    let jwt_service = Arc::new(JwtServiceImpl::new(Arc::clone(&config_provider)));

    // Determine which repository to use based on environment variable
    let use_memory_repo = std::env::var("USE_MEMORY_REPO")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);

    // Create application services with the appropriate repository
    let auth_service = if use_memory_repo {
        info!("Using in-memory repository");
        Arc::new(application::services::AuthServiceImpl::new(
            Arc::clone(&memory_user_repo),
            Arc::clone(&jwt_service),
            Arc::clone(&password_service),
        ))
    } else {
        info!("Using PostgreSQL repository");
        Arc::new(application::services::AuthServiceImpl::new(
            Arc::clone(&postgres_user_repo),
            Arc::clone(&jwt_service),
            Arc::clone(&password_service),
        ))
    };

    let user_service = if use_memory_repo {
        Arc::new(application::services::UserServiceImpl::new(
            Arc::clone(&memory_user_repo),
            Arc::clone(&password_service),
        ))
    } else {
        Arc::new(application::services::UserServiceImpl::new(
            Arc::clone(&postgres_user_repo),
            Arc::clone(&password_service),
        ))
    };

    // Create use cases
    let auth_use_cases = Arc::new(application::use_cases::AuthUseCases::new(
        Arc::clone(&auth_service),
    ));

    let user_use_cases = Arc::new(application::use_cases::UserUseCases::new(
        Arc::clone(&user_service),
    ));

    // Build the application state
    let app_state = api::AppState {
        auth_use_cases,
        user_use_cases,
        jwt_service: Arc::clone(&jwt_service),
        config_provider: Arc::clone(&config_provider),
    };

    // Build the router
    let app = api::create_router(app_state);

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("Server shutdown complete");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, starting graceful shutdown");
        },
        _ = terminate => {
            info!("Received terminate signal, starting graceful shutdown");
        },
    }
}
