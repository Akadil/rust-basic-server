use crate::{handlers::get_protected_data, middleware::auth};
use axum::{middleware::from_fn, routing::get, Router};

pub fn create_protected_router() -> Router {
    Router::new()
        .route("/protected", get(get_protected_data))
        .layer(from_fn(auth))
}
