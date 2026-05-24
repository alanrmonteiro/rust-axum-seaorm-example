#[cfg(test)]
use std::sync::Arc;

#[cfg(test)]
use crate::api::setup_routes;
#[cfg(test)]
use crate::infra::auth::auth::{AppState, get_jwks_client};
#[cfg(test)]
use crate::infra::utils::db::setup_test_db_with_schema;
#[cfg(test)]
use axum_test::TestServer;
#[cfg(test)]
use jwks::Jwks;

#[cfg(test)]
pub async fn setup_api_tests_infra() -> (TestServer, sea_orm::DatabaseConnection) {
    let db = setup_test_db_with_schema().await;
    let jwks_client: Jwks = get_jwks_client().await;
    let app_state: Arc<AppState> = Arc::new(AppState {
        jwks_client: jwks_client,
        db_conn: db.clone(),
    });
    let app = setup_routes(app_state);
    let server = TestServer::new(app);
    (server, db)
}
