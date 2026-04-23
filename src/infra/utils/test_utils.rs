#[cfg(test)]
use crate::api::setup_routes;
#[cfg(test)]
use crate::infra::utils::db::setup_test_db_with_schema;
#[cfg(test)]
use axum_test::TestServer;

#[cfg(test)]
pub async fn setup_api_tests_infra() -> (TestServer, sea_orm::DatabaseConnection) {
    let db = setup_test_db_with_schema().await;
    let app = setup_routes(db.clone());
    let server = TestServer::new(app);
    (server, db)
}
