#[cfg(test)]
use sea_orm::{ConnectionTrait, EntityTrait, Schema};

#[cfg(test)]
use crate::domain;

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
#[cfg(test)]
pub async fn setup_test_db() -> DatabaseConnection {
    let db = Database::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to test database");
    db
}

// Chamamos passando o tipo: create_table::<tasks::Entity>(&db).await;
#[cfg(test)]
pub async fn create_table<T>(db: &DatabaseConnection)
where
    T: EntityTrait + Default,
{
    let backend = db.get_database_backend();
    let schema = Schema::new(backend);

    // T::default() funciona porque EntityTrait exige que a entidade seja Default
    let stmt = backend.build(&schema.create_table_from_entity(T::default()));
    db.execute(stmt).await.unwrap();
}

#[cfg(test)]
pub async fn setup_test_db_with_schema() -> DatabaseConnection {
    let db = setup_test_db().await;
    create_table::<domain::tasks::Entity>(&db).await;
    create_table::<domain::users::Entity>(&db).await;
    db
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_db_pool_success() {
        let db_url = "sqlite::memory:";
        let db = init_db_pool(db_url).await;

        assert!(
            db.ping().await.is_ok(),
            "Database connection should be active"
        );

        let backend = db.get_database_backend();
        let res = db
            .execute(sea_orm::Statement::from_string(
                backend,
                "SELECT 1".to_string(),
            ))
            .await;

        assert!(res.is_ok(), "Should be able to execute a simple query");
    }
}
