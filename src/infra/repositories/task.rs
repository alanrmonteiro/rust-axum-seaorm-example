use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, prelude::async_trait,
};

use crate::{
    api::handlers::task::{CreateTaskRequest, TaskFilter},
    domain::tasks::{self, TaskFilterExt},
    infra::utils::error::AppError,
    services::task::get_task_by_id,
};

#[async_trait::async_trait]
pub trait TaskRepository: Send + Sync {
    async fn create_task(&self, task: CreateTaskRequest) -> Result<tasks::ActiveModel, AppError>;
    async fn get_task_by_id(&self, id: i32) -> Result<tasks::Model, AppError>;
    async fn hard_remove_task_by_id(&self, id: i32) -> Result<(), AppError>;
    async fn soft_remove_task_by_id(&self, id: i32) -> Result<(), AppError>;
    async fn list_tasks_by_filter(&self, filter: TaskFilter)
    -> Result<Vec<tasks::Model>, AppError>;
    async fn update_task(&self, id: i32, task: CreateTaskRequest)
    -> Result<tasks::Model, AppError>;
    async fn link_user_to_task(&self, user_id: i32, task_id: i32)
    -> Result<tasks::Model, AppError>;
}

pub struct TaskRepositoryImpl {
    db_conn: DatabaseConnection,
}

impl TaskRepositoryImpl {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self { db_conn }
    }
}

#[async_trait::async_trait]
impl TaskRepository for TaskRepositoryImpl {
    async fn create_task(&self, task: CreateTaskRequest) -> Result<tasks::ActiveModel, AppError> {
        let new_task = tasks::ActiveModel {
            priority: Set(task.priority),
            title: Set(task.title),
            description: Set(task.description),
            ..Default::default()
        };
        new_task
            .save(&self.db_conn)
            .await
            .map_err(|err| AppError::InternalError(err.to_string()))
    }

    async fn get_task_by_id(&self, id: i32) -> Result<tasks::Model, AppError> {
        let task: Option<tasks::Model> = tasks::Entity::find()
            .filter(tasks::Column::Id.eq(id))
            .filter(tasks::Column::DeletedAt.is_null())
            .one(&self.db_conn)
            .await
            .map_err(|err| AppError::InternalError(err.to_string()))?;

        task.ok_or_else(|| AppError::NotFound(format!("Task with id {} not found", id)))
    }

    async fn hard_remove_task_by_id(&self, id: i32) -> Result<(), AppError> {
        let res = tasks::Entity::delete_by_id(id)
            .exec(&self.db_conn)
            .await
            .map_err(|e| {
                AppError::InternalError(format!("Failed to delete task: {}", e.to_string()))
            })?;

        if res.rows_affected == 0 {
            return Err(AppError::NotFound(format!("Task with id {} not found", id)));
        }
        Ok(())
    }

    async fn soft_remove_task_by_id(&self, id: i32) -> Result<(), AppError> {
        let mut task: tasks::ActiveModel =
            get_task_by_id(&self.db_conn, id).await?.into_active_model();

        task.deleted_at = Set(Some(chrono::Utc::now().into()));
        task.update(&self.db_conn)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(())
    }

    async fn list_tasks_by_filter(
        &self,
        filter: TaskFilter,
    ) -> Result<Vec<tasks::Model>, AppError> {
        let tasks = tasks::Entity::find()
            .filter_by_fields(filter)
            .filter(tasks::Column::DeletedAt.is_null())
            .all(&self.db_conn)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;
        Ok(tasks)
    }
    async fn update_task(
        &self,
        id: i32,
        task: CreateTaskRequest,
    ) -> Result<tasks::Model, AppError> {
        let task_db = get_task_by_id(&self.db_conn, id).await?;

        let mut active_model: tasks::ActiveModel = task_db.into();
        active_model.priority = Set(task.priority);
        active_model.title = Set(task.title);
        active_model.description = Set(task.description);

        active_model
            .update(&self.db_conn)
            .await
            .map_err(|err| AppError::InternalError(err.to_string()))
    }

    async fn link_user_to_task(
        &self,
        user_id: i32,
        task_id: i32,
    ) -> Result<tasks::Model, AppError> {
        let mut task = get_task_by_id(&self.db_conn, task_id).await?;

        task.user_id = Some(user_id);
        let active_model: tasks::ActiveModel = task.into();

        active_model
            .update(&self.db_conn)
            .await
            .map_err(|err| AppError::InternalError(err.to_string()))
    }
}
