use axum::{
    routing::get,
    Router,
};

use crate::{app::{commands::create_user_command::create_user_command, queries::get_user_query::get_user_query}, infra::repositories::user_repository::UserRepository};


pub fn create_router(repo: UserRepository) -> Router {
    Router::new()
        .route("/api/user", get(get_user_query).post(create_user_command))
        .layer(axum::Extension(repo))
}
