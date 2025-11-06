use std::env;
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};

pub async fn init_db() -> DatabaseConnection{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connect = Database::connect(&database_url).await;
    connect.expect("REASON")
}
