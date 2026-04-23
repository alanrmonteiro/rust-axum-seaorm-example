use crate::{api::handlers::user::CreateUserRequest, infra::utils::error::AppError};

pub fn validate_request(req: &CreateUserRequest) -> Result<(), AppError> {
    req.user_name
        .as_ref()
        .map(|n| n.as_str())
        .filter(|n| !n.is_empty())
        .ok_or_else(|| AppError::BadRequest("Name is required".to_string()))?;

    req.password
        .as_ref()
        .map(|p| p.as_str())
        .filter(|p| !p.is_empty())
        .ok_or_else(|| AppError::BadRequest("Password is required".to_string()))?;
    Ok(())
}
