use crate::{
    error::{AppError, Result},
    utils::get_user_id_from_token,
};
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

pub async fn auth<B>(
    req: Request<B>,
    next: Next<B>,
) -> std::result::Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    let token = match auth_header {
        Some(auth) if auth.starts_with("Bearer ") => {
            let token = auth.trim_start_matches("Bearer ").trim();
            token
        }
        _ => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Verify the token and extract user_id
    match get_user_id_from_token(token) {
        Ok(user_id) => {
            // Store the user_id in request extensions for handlers to access
            let mut req = req;
            req.extensions_mut().insert(user_id);
            
            // Continue to the handler
            Ok(next.run(req).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
