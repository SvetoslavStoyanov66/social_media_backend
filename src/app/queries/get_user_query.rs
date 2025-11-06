use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::infra::repositories::user_repository;

pub async fn get_user_query(
    Extension(repo): Extension<user_repository::UserRepository>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let users = match repo.get_all().await {
        Ok(users) => users,
        Err(err) => {
            let json_response = json!({
                "status": "error",
                "message": err.to_string()
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)));
        }
    };

    let json_response = json!({
        "status": "success",
        "data": users
    });

    Ok((StatusCode::OK, Json(json_response)))
}
