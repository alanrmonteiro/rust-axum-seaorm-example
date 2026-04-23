use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, prelude::async_trait};

use crate::{api::handlers::user::CreateUserRequest, domain::users, infra::utils::error::AppError};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: CreateUserRequest) -> Result<users::ActiveModel, AppError>;
}

pub struct UserRepositoryImpl {
    db_conn: DatabaseConnection,
}

impl UserRepositoryImpl {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self { db_conn }
    }
}

#[async_trait::async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create_user(&self, user: CreateUserRequest) -> Result<users::ActiveModel, AppError> {
        let new_user = users::ActiveModel {
            username: Set(user.user_name.unwrap()),
            password: Set(user.password.unwrap()),
            token: Set(user.token.or_else(|| Some("".to_string()))),
            ..Default::default()
        };
        new_user
            .save(&self.db_conn)
            .await
            .map_err(|err| AppError::InternalError(err.to_string()))
    }
}
