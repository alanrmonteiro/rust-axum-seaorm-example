mod api;
mod domain;
mod infra;
mod services;
use api::setup_routes;
use dotenvy::dotenv;
use infra::utils::db::init_db_pool;
use infra::utils::http_server::run_http_server;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = get_db_url();
    let db = init_db_pool(&database_url).await;
    let app = setup_routes(db);
    run_http_server(app).await;
}

fn get_db_url() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL not set")
}
