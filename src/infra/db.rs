// src/infrastructure/db.rs
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub async fn init_db_pool(database_url: &str) -> DatabaseConnection {
    let mut options = ConnectOptions::new(database_url.to_string());

    options
        .max_connections(20)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(600))
        .sqlx_logging(true);

    Database::connect(options)
        .await
        .expect("Failed to connect to database")
}
