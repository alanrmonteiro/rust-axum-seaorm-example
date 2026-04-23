pub mod adapters;
pub mod request_validation;

use axum::extract::Query;
use axum::{Extension, Json, extract::Path};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::api::handlers::task::adapters::to_create_task_response;
use crate::api::handlers::task::adapters::to_get_task_response;
use crate::api::handlers::task::request_validation::validate_request;
use crate::infra::utils::error::AppError;
use crate::services::task::{
    get_task_by_id, hard_remove_task_by_id, sorft_remove_task_by_id, update_task,
};

use crate::services::task::{create_task_service, list_tasks_by_filter};

#[derive(serde::Deserialize)]
pub struct CreateTaskRequest {
    pub priority: Option<String>,
    pub title: String,
    pub description: Option<String>,
}
#[derive(serde::Serialize)]
pub struct CreateTaskResponse {
    pub id: i32,
    pub priority: Option<String>,
    pub title: String,
    pub description: Option<String>,
}

#[derive(serde::Serialize, Debug, PartialEq, Deserialize)]
pub struct GetTaskResponse {
    pub id: i32,
    pub priority: Option<String>,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TaskFilter {
    pub priority: Option<String>,
    pub user_id: Option<i32>,
    pub is_default: Option<bool>,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(serde::Serialize, Debug)]
pub struct DeleteTaskResponse {
    pub id: i32,
    pub message: String,
}

pub async fn create_task(
    Extension(db_conn): Extension<DatabaseConnection>,
    Json(req): Json<CreateTaskRequest>,
) -> Result<Json<CreateTaskResponse>, AppError> {
    validate_request(&req)?;
    let task = create_task_service(&db_conn, req).await?;
    Ok(Json(to_create_task_response(task)))
}

pub async fn get_by_id(
    Path(task_id): Path<i32>,
    Extension(db_conn): Extension<DatabaseConnection>,
) -> Result<Json<GetTaskResponse>, AppError> {
    let task = get_task_by_id(&db_conn, task_id).await?;
    Ok(Json(to_get_task_response(task)))
}

pub async fn get_all_tasks(
    Extension(db_conn): Extension<DatabaseConnection>,
    Query(filter): Query<TaskFilter>,
) -> Result<Json<Vec<GetTaskResponse>>, AppError> {
    let tasks: Vec<crate::domain::tasks::Model> = list_tasks_by_filter(&db_conn, filter).await?;
    let tasks_response = tasks.into_iter().map(to_get_task_response).collect();
    Ok(Json(tasks_response))
}

pub async fn put_task(
    Path(task_id): Path<i32>,
    Extension(db_conn): Extension<DatabaseConnection>,
    Json(req): Json<CreateTaskRequest>,
) -> Result<Json<GetTaskResponse>, AppError> {
    validate_request(&req)?;
    let task = update_task(&db_conn, task_id, req).await?;
    Ok(Json(to_get_task_response(task)))
}

pub async fn link_user_to_task(
    Path((user_id, task_id)): Path<(i32, i32)>,
    Extension(db_conn): Extension<DatabaseConnection>,
) -> Result<Json<GetTaskResponse>, AppError> {
    let task = crate::services::task::link_user_to_task(&db_conn, user_id, task_id).await?;
    Ok(Json(to_get_task_response(task)))
}

pub async fn hard_delete_by_id(
    Path(task_id): Path<i32>,
    Extension(db_conn): Extension<DatabaseConnection>,
) -> Result<Json<DeleteTaskResponse>, AppError> {
    hard_remove_task_by_id(&db_conn, task_id).await?;

    Ok(Json(DeleteTaskResponse {
        id: task_id,
        message: "Task deleted successfully".to_string(),
    }))
}

pub async fn soft_delete_by_id(
    Path(task_id): Path<i32>,
    Extension(db_conn): Extension<DatabaseConnection>,
) -> Result<Json<DeleteTaskResponse>, AppError> {
    sorft_remove_task_by_id(&db_conn, task_id).await?;

    Ok(Json(DeleteTaskResponse {
        id: task_id,
        message: "Task deleted successfully".to_string(),
    }))
}
