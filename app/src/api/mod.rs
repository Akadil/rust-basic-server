pub mod auth;
pub mod users;

use std::sync::Arc;

use application::use_cases::{AuthUseCases, UserUseCases};
use application::services::JwtService;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use infrastructure::config::ConfigProvider;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use crate::middleware::{
    auth::{auth_middleware, AuthState},
    tracing::{create_tracing_layer, request_tracing_middleware},
};

#[derive(Clone)]
pub struct AppState {
    pub auth_use_cases: Arc<AuthUseCases>,
    pub user_use_cases: Arc<UserUseCases>,
    pub jwt_service: Arc<dyn JwtService>,
    pub config_provider: Arc<dyn ConfigProvider>,
}

pub fn create_router(app_state: AppState) -> Router {
    info!("Creating application router");

    // Create the auth state for the auth middleware
    let auth_state = AuthState {
        jwt_service: Arc::clone(&app_state.jwt_service),
    };

    // Create the CORS layer
    let config = app_state.config_provider.get_config();
    let cors_layer = CorsLayer::new()
        .allow_origin(config.cors.allowed_origins.iter().map(|origin| origin.parse().unwrap()).collect::<Vec<_>>())
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_credentials(config.cors.allow_credentials);

    // Create the router
    Router::new()
        // API routes
        .nest(
            "/api",
            Router::new()
                // Auth routes (no authentication required)
                .nest(
                    "/auth",
                    Router::new()
                        .route("/login", post(auth::login))
                        .route("/register", post(auth::register)),
                )
                // User routes (authentication required)
                .nest(
                    "/users",
                    Router::new()
                        .route("/", get(users::get_all_users).post(users::create_user))
                        .route("/:id", get(users::get_user).put(users::update_user).delete(users::delete_user))
                        .route_layer(middleware::from_fn_with_state(
                            auth_state.clone(),
                            auth_middleware,
                        )),
                ),
        )
        // Health check route
        .route("/health", get(health_check))
        // Add middleware
        .layer(create_tracing_layer())
        .layer(middleware::from_fn(request_tracing_middleware))
        .layer(cors_layer)
        .with_state(app_state)
}

async fn health_check() -> &'static str {
    "OK"
}
