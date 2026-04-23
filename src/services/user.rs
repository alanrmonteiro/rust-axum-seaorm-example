use sea_orm::DatabaseConnection;

use crate::{
    api::handlers::user::CreateUserRequest,
    domain::users,
    infra::{
        repositories::user::{UserRepository, UserRepositoryImpl},
        utils::error::AppError,
    },
};

pub async fn create_user_service(
    db_conn: &DatabaseConnection,
    req: CreateUserRequest,
) -> Result<users::ActiveModel, AppError> {
    UserRepositoryImpl::new(db_conn.clone())
        .create_user(req)
        .await
}
