use crate::handlers::{login, register};
use axum::routing::post;
use axum::Router;

pub fn create_auth_router() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
