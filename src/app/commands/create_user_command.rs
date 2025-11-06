use axum::{Extension, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::infra::repositories::user_repository::UserRepository;

pub async fn create_user_command(
    Extension(repo): Extension<UserRepository>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<CreateUserResponse>), (StatusCode, Json<serde_json::Value>)> {
    match repo.create_user(payload.username).await {
        Ok(user) => Ok((StatusCode::CREATED, Json(user.into()))),
        Err(err) => {
            let json_response = json!({
                "status": "error",
                "message": err.to_string()
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)))
        }
    }
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    username: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    id: i32,
    username: String,
}

impl From<crate::domain::models::user::Model> for CreateUserResponse {
    fn from(user: crate::domain::models::user::Model) -> Self {
        Self {
            id: user.id,
            username: user.username,
        }
    }
}
