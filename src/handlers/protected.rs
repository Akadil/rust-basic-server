use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use uuid::Uuid;

// Example of a protected route handler that requires authentication
pub async fn get_protected_data(
    Extension(user_id): Extension<Uuid>,
) -> impl IntoResponse {
    // The user_id is extracted from the JWT token by the auth middleware
    // We can use it to fetch user-specific data or perform authorized actions
    
    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "You have access to protected data",
            "data": {
                "user_id": user_id.to_string(),
                "secret_info": "This is protected information that only authenticated users can see"
            }
        })),
    )
}
