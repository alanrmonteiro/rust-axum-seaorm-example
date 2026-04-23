pub mod adapters;
pub mod request_validation;

use crate::{
    api::handlers::user::adapters::to_create_user_response, infra::utils::error::AppError,
    services::user::create_user_service,
};
use axum::{Json, extract::Extension};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

#[derive(serde::Deserialize)]
pub struct CreateUserRequest {
    pub user_name: Option<String>,
    pub token: Option<String>,
    pub password: Option<String>,
}

#[derive(serde::Serialize, Debug, PartialEq, Deserialize)]
pub struct CreateUserResponse {
    pub id: i32,
    pub user_name: String,
    pub password: String,
}

pub async fn create_user(
    Extension(db_conn): Extension<DatabaseConnection>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, AppError> {
    request_validation::validate_request(&req)?;
    let user = create_user_service(&db_conn, req).await?;
    Ok(Json(to_create_user_response(user)))
}
