use crate::api::handlers::task::TaskFilter;
use crate::domain::tasks::{self};
use crate::infra::repositories::task::{TaskRepository, TaskRepositoryImpl};
use crate::{api::handlers::task::CreateTaskRequest, infra::utils::error::AppError};

use sea_orm::DatabaseConnection;

pub async fn create_task_service(
    db_conn: &DatabaseConnection,
    req: CreateTaskRequest,
) -> Result<tasks::ActiveModel, AppError> {
    TaskRepositoryImpl::new(db_conn.clone())
        .create_task(req)
        .await
}

pub async fn get_task_by_id(
    db_conn: &DatabaseConnection,
    id: i32,
) -> Result<tasks::Model, AppError> {
    TaskRepositoryImpl::new(db_conn.clone())
        .get_task_by_id(id)
        .await
}

pub async fn hard_remove_task_by_id(db_conn: &DatabaseConnection, id: i32) -> Result<(), AppError> {
    TaskRepositoryImpl::new(db_conn.clone())
        .hard_remove_task_by_id(id)
        .await
}

pub async fn sorft_remove_task_by_id(
    db_conn: &DatabaseConnection,
    id: i32,
) -> Result<(), AppError> {
    TaskRepositoryImpl::new(db_conn.clone())
        .soft_remove_task_by_id(id)
        .await
}

pub async fn list_tasks_by_filter(
    db_conn: &DatabaseConnection,
    filter: TaskFilter,
) -> Result<Vec<tasks::Model>, AppError> {
    TaskRepositoryImpl::new(db_conn.clone())
        .list_tasks_by_filter(filter)
        .await
}

pub async fn update_task(
    db_conn: &DatabaseConnection,
    id: i32,
    req: CreateTaskRequest,
) -> Result<tasks::Model, AppError> {
    TaskRepositoryImpl::new(db_conn.clone())
        .update_task(id, req)
        .await
}
pub async fn link_user_to_task(
    db_conn: &DatabaseConnection,
    user_id: i32,
    task_id: i32,
) -> Result<tasks::Model, AppError> {
    TaskRepositoryImpl::new(db_conn.clone())
        .link_user_to_task(user_id, task_id)
        .await
}
