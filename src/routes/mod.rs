pub mod auth;
pub mod protected;

use axum::Router;

pub fn create_router() -> Router {
    Router::new()
        .nest("/api/auth", auth::create_auth_router())
        .nest("/api", protected::create_protected_router())
        .route("/", axum::routing::get(|| async { "Rust Basic Server with Axum and JWT Authentication" }))
}
