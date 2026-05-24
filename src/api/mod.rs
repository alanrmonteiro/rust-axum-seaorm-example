pub mod handlers;

use std::sync::Arc;

use axum::{
    Router,
    http::Method,
    routing::{delete, get, post, put},
};

use tower_http::cors::{Any, CorsLayer};

use crate::api::handlers::task::{
    create_task, get_all_tasks, get_by_id, hard_delete_by_id, link_user_to_task, put_task,
    soft_delete_by_id,
};
use crate::api::handlers::user::create_user;
use crate::infra::auth::auth::AppState;

pub fn setup_routes(app_state: Arc<AppState>) -> Router {
    let cors = build_cors_layer();

    Router::new()
        .route("/task", post(create_task))
        .route("/task", get(get_all_tasks))
        .route("/task/{id}", get(get_by_id))
        .route("/task/{id}", put(put_task))
        .route("/task/hard/{id}", delete(hard_delete_by_id))
        .route("/task/{id}", delete(soft_delete_by_id))
        .route("/task/user/{user_id}/{task_id}", put(link_user_to_task))
        .route("/user", post(create_user))
        .layer(cors)
        .with_state(app_state)
}

pub fn build_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
}

#[cfg(test)]
mod tests {

    use crate::{api::handlers::user::CreateUserResponse, domain::tasks};
    use sea_orm::ActiveModelTrait;
    use sea_orm::ActiveValue::Set;
    use serde_json::json;

    use crate::{
        api::handlers::task::GetTaskResponse, domain::tasks::ActiveModel,
        infra::utils::test_utils::setup_api_tests_infra,
    };
    #[tokio::test]
    async fn test_get_one_task() {
        let (server, db) = setup_api_tests_infra().await;

        let seed_task = ActiveModel {
            title: Set("Task de Teste".to_owned()),
            description: Set(Some("Descrição da task".to_owned())),
            priority: Set(Some("High".to_owned())),
            ..Default::default()
        };

        let created_task: tasks::Model = seed_task
            .insert(&db)
            .await
            .expect("Erro ao inserir task de teste");

        let response = server.get("/task").await;

        response.assert_status_ok();
        response.assert_json(&vec![GetTaskResponse {
            id: created_task.id,
            title: created_task.title,
            description: created_task.description,
            priority: created_task.priority,
        }]);
    }

    #[tokio::test]
    async fn test_get_all_task() {
        let (server, db) = setup_api_tests_infra().await;

        let seed_task = ActiveModel {
            title: Set("Task de Teste".to_owned()),
            description: Set(Some("Descrição da task".to_owned())),
            priority: Set(Some("High".to_owned())),
            ..Default::default()
        };

        let seed_task2 = ActiveModel {
            title: Set("Task de Teste 2".to_owned()),
            description: Set(Some("Descrição da task 2".to_owned())),
            priority: Set(Some("Medium".to_owned())),
            ..Default::default()
        };

        let created_task: tasks::Model = seed_task
            .insert(&db)
            .await
            .expect("Erro ao inserir task de teste");

        let created_task2: tasks::Model = seed_task2
            .insert(&db)
            .await
            .expect("Erro ao inserir task de teste");

        let response = server.get("/task").await;

        response.assert_status_ok();
        response.assert_status_success();

        let tasks: Vec<GetTaskResponse> = response.json();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].id, created_task.id);
        assert_eq!(tasks[0].title, created_task.title);
        assert_eq!(tasks[0].description, created_task.description);
        assert_eq!(tasks[0].priority, created_task.priority);
        assert_eq!(tasks[1].id, created_task2.id);
        assert_eq!(tasks[1].title, created_task2.title);
        assert_eq!(tasks[1].description, created_task2.description);
        assert_eq!(tasks[1].priority, created_task2.priority);
    }

    #[tokio::test]
    async fn test_get_all_task_by_filter() {
        let (server, db) = setup_api_tests_infra().await;

        let seed_task = ActiveModel {
            title: Set("Task de Teste".to_owned()),
            description: Set(Some("Descrição da task".to_owned())),
            priority: Set(Some("High".to_owned())),
            ..Default::default()
        };

        let seed_task2 = ActiveModel {
            title: Set("Task de Teste 2".to_owned()),
            description: Set(Some("Descrição da task 2".to_owned())),
            priority: Set(Some("Medium".to_owned())),
            ..Default::default()
        };

        let created_task: tasks::Model = seed_task
            .insert(&db)
            .await
            .expect("Erro ao inserir task de teste");

        let _created_task2: tasks::Model = seed_task2
            .insert(&db)
            .await
            .expect("Erro ao inserir task de teste");

        let response = server
            .get("/task?priority=High&title=Teste&description=task")
            .await;

        response.assert_status_ok();
        response.assert_status_success();

        let tasks: Vec<GetTaskResponse> = response.json();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, created_task.id);
        assert_eq!(tasks[0].title, created_task.title);
        assert_eq!(tasks[0].description, created_task.description);
        assert_eq!(tasks[0].priority, created_task.priority);
    }

    #[tokio::test]
    async fn test_post_one_task() {
        let (server, _db) = setup_api_tests_infra().await;

        let payload = json!({
            "title": "Aprender Integração em Rust",
            "description": "Finalizar os testes de POST",
            "priority": "High",
        });
        let response = server.post("/task").json(&payload).await;

        response.assert_status_ok();
        response.assert_json(&GetTaskResponse {
            id: 1,
            title: "Aprender Integração em Rust".to_owned(),
            description: Some("Finalizar os testes de POST".to_owned()),
            priority: Some("High".to_owned()),
        });
    }

    #[tokio::test]
    async fn test_post_one_task_missing_required_fields() {
        let (server, _db) = setup_api_tests_infra().await;

        let payload = json!({
            "title": "Aprender Integração em Rust",
            "description": "Finalizar os testes de POST",
            // "priority" is missing, but it's optional, so it should still work
        });
        let response = server.post("/task").json(&payload).await;

        response.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_put_one_task() {
        let (server, db) = setup_api_tests_infra().await;

        let seed_task = ActiveModel {
            title: Set("Task de Teste".to_owned()),
            description: Set(Some("Descrição da task".to_owned())),
            priority: Set(Some("High".to_owned())),
            ..Default::default()
        };

        seed_task
            .insert(&db)
            .await
            .expect("Erro ao inserir task de teste");

        let payload = json!({
            "title": "Updated Title",
            "description": "Updated Description",
            "priority": "Updated Priority",
        });
        let response = server.put("/task/1").json(&payload).await;

        response.assert_status_ok();
        response.assert_json(&GetTaskResponse {
            id: 1,
            title: "Updated Title".to_owned(),
            description: Some("Updated Description".to_owned()),
            priority: Some("Updated Priority".to_owned()),
        });
    }
    #[tokio::test]
    async fn test_hard_delete_one_task() {
        let (server, db) = setup_api_tests_infra().await;
        let seed_task = ActiveModel {
            title: Set("Task de Teste".to_owned()),
            description: Set(Some("Descrição da task".to_owned())),
            priority: Set(Some("High".to_owned())),
            ..Default::default()
        };

        seed_task
            .insert(&db)
            .await
            .expect("Erro ao inserir task de teste");

        let response = server.delete("/task/hard/1").await;
        response.assert_status_ok();
        let get_response = server.get("/task/1").await;
        get_response.assert_status_not_found();
    }

    #[tokio::test]
    async fn test_hard_delete_unexisting_task() {
        let (server, db) = setup_api_tests_infra().await;
        let seed_task = ActiveModel {
            title: Set("Task de Teste".to_owned()),
            description: Set(Some("Descrição da task".to_owned())),
            priority: Set(Some("High".to_owned())),
            ..Default::default()
        };

        seed_task
            .insert(&db)
            .await
            .expect("Erro ao inserir task de teste");

        let response = server.delete("/task/hard/55").await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn test_soft_delete_one_task() {
        let (server, db) = setup_api_tests_infra().await;
        let seed_task = ActiveModel {
            title: Set("Task de Teste".to_owned()),
            description: Set(Some("Descrição da task".to_owned())),
            priority: Set(Some("High".to_owned())),
            ..Default::default()
        };

        seed_task
            .insert(&db)
            .await
            .expect("Erro ao inserir task de teste");

        let response = server.delete("/task/1").await;
        response.assert_status_ok();
        let get_response = server.get("/task/1").await;
        get_response.assert_status_not_found();
    }

    #[tokio::test]
    async fn link_user_to_task() {
        let (server, db) = setup_api_tests_infra().await;
        let seed_task = ActiveModel {
            title: Set("Task de Teste".to_owned()),
            description: Set(Some("Descrição da task".to_owned())),
            priority: Set(Some("High".to_owned())),
            ..Default::default()
        };

        let saved_task = seed_task
            .clone()
            .insert(&db)
            .await
            .expect("Erro ao inserir task de teste");

        let seed_user = crate::domain::users::ActiveModel {
            username: Set("John Doe".to_owned()),
            password: Set("password123".to_owned()),
            token: Set(Some("some_token".to_owned())),
            ..Default::default()
        };

        let saved_user = seed_user
            .clone()
            .insert(&db)
            .await
            .expect("Erro ao inserir user de teste");

        let response = server
            .put(format!("/task/user/{}/{}", saved_task.id, saved_user.id).as_str())
            .await;

        response.assert_status_ok();
    }

    #[tokio::test]
    async fn test_post_one_user() {
        let (server, _db) = setup_api_tests_infra().await;

        let payload = json!({
            "user_name": "john_doe",
            "password": "secret123",
            "token": "some_token"
        });
        let response = server.post("/user").json(&payload).await;

        response.assert_status_ok();
        response.assert_json(&CreateUserResponse {
            id: 1,
            user_name: "john_doe".to_owned(),
            password: "secret123".to_owned(),
        });
    }
    #[tokio::test]
    async fn test_post_one_invalid_user() {
        let (server, _db) = setup_api_tests_infra().await;

        let payload = json!({
            "token": "some_token"
        });
        let response = server.post("/user").json(&payload).await;

        let payload2 = json!({
            "user_name": "john_doe",
            "token": "some_token"
        });
        let response2 = server.post("/user").json(&payload2).await;

        response.assert_status_bad_request();
        response2.assert_status_bad_request();
    }
}
