pub mod handlers;
use axum::{
    Extension, Router,
    http::Method,
    routing::{delete, get, post, put},
};

use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

use crate::api::handlers::task::{
    create_task, get_all_tasks, get_by_id, hard_delete_by_id, put_task, soft_delete_by_id,
};

pub fn setup_routes(db_conn: DatabaseConnection) -> Router {
    let cors = build_cors_layer();

    Router::new()
        .route("/task", post(create_task))
        .route("/task", get(get_all_tasks))
        .route("/task/{id}", get(get_by_id))
        .route("/task/{id}", put(put_task))
        .route("/task/hard/{id}", delete(hard_delete_by_id))
        .route("/task/{id}", delete(soft_delete_by_id))
        .layer(Extension(db_conn))
        .layer(cors)
}

pub fn build_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
}
