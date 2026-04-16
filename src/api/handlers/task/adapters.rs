use crate::api::handlers::task::{CreateTaskResponse, GetTaskResponse};

pub fn to_create_task_response(task: crate::domain::tasks::ActiveModel) -> CreateTaskResponse {
    CreateTaskResponse {
        id: task.id.unwrap(),
        priority: task.priority.unwrap(),
        title: task.title.unwrap(),
        description: task.description.unwrap(),
    }
}
pub fn to_get_task_response(task: crate::domain::tasks::Model) -> GetTaskResponse {
    GetTaskResponse {
        id: task.id,
        priority: task.priority,
        title: task.title,
        description: task.description,
    }
}
