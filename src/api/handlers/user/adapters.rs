use crate::api::handlers::user::CreateUserResponse;

pub fn to_create_user_response(user: crate::domain::users::ActiveModel) -> CreateUserResponse {
    CreateUserResponse {
        id: user.id.unwrap(),
        user_name: user.username.unwrap(),
        password: user.password.unwrap(),
    }
}
