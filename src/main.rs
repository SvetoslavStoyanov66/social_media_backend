use social_media_backend::{api::router::create_router, infra::{db_context::postgres_context::init_db, repositories::user_repository::{UserRepository}}};


#[tokio::main]
async fn main() {
    let db = init_db().await;

    let repo = UserRepository::new(db);

    let app = create_router(repo);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
