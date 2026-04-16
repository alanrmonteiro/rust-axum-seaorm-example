use crate::api::handlers::task::TaskFilter;
use crate::domain::tasks::{self, TaskFilterExt};
use crate::{api::handlers::task::CreateTaskRequest, infra::error::AppError};

use sea_orm::IntoActiveModel;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};

pub async fn create_task_service(
    db_conn: &DatabaseConnection,
    req: CreateTaskRequest,
) -> Result<tasks::ActiveModel, AppError> {
    let new_task = tasks::ActiveModel {
        priority: Set(req.priority),
        title: Set(req.title),
        description: Set(req.description),
        ..Default::default()
    };
    new_task
        .save(db_conn)
        .await
        .map_err(|err| AppError::InternalError(err.to_string()))
}

pub async fn get_task_by_id(
    db_conn: &DatabaseConnection,
    id: i32,
) -> Result<tasks::Model, AppError> {
    let task: Option<tasks::Model> = tasks::Entity::find_by_id(id)
        .one(db_conn)
        .await
        .map_err(|err| AppError::InternalError(err.to_string()))?;

    task.ok_or_else(|| AppError::NotFound(format!("Task with id {} not found", id)))
}

pub async fn hard_remove_task_by_id(db_conn: &DatabaseConnection, id: i32) -> Result<(), AppError> {
    let res = tasks::Entity::delete_by_id(id)
        .exec(db_conn)
        .await
        .map_err(|err| AppError::InternalError(err.to_string()))?;

    if res.rows_affected == 0 {
        return Err(AppError::NotFound("Nada foi deletado".into()));
    }
    Ok(())
}

pub async fn sorft_remove_task_by_id(
    db_conn: &DatabaseConnection,
    id: i32,
) -> Result<(), AppError> {
    let mut task = get_task_by_id(db_conn, id).await?.into_active_model();

    task.deleted_at = Set(Some(chrono::Utc::now().into()));
    task.update(db_conn)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    Ok(())
}

pub async fn list_tasks_by_filter(
    db_conn: &DatabaseConnection,
    filter: TaskFilter,
) -> Result<Vec<tasks::Model>, AppError> {
    let tasks = tasks::Entity::find()
        .filter_by_fields(filter)
        .all(db_conn)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;
    Ok(tasks)
}

pub async fn update_task(
    db_conn: &DatabaseConnection,
    id: i32,
    req: CreateTaskRequest,
) -> Result<tasks::Model, AppError> {
    let task = get_task_by_id(db_conn, id).await?;

    let mut active_model: tasks::ActiveModel = task.into();
    active_model.priority = Set(req.priority);
    active_model.title = Set(req.title);
    active_model.description = Set(req.description);

    active_model
        .update(db_conn)
        .await
        .map_err(|err| AppError::InternalError(err.to_string()))
}
