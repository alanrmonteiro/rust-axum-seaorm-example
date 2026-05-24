mod api;
mod domain;
mod infra;
mod services;
use std::sync::Arc;

use api::setup_routes;
use dotenvy::dotenv;
use infra::utils::db::init_db_pool;
use infra::utils::http_server::run_http_server;

use crate::infra::auth::auth::get_jwks_client;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = get_db_url();
    let db_conn = init_db_pool(&database_url).await;

    let jwks_client: jwks::Jwks = get_jwks_client().await;
    let app_state = Arc::new(infra::auth::auth::AppState {
        jwks_client,
        db_conn: db_conn.clone(),
    });

    let app = setup_routes(app_state);
    run_http_server(app).await;
}

fn get_db_url() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL not set")
}
