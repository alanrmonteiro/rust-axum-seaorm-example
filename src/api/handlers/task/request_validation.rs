use crate::{api::handlers::task::CreateTaskRequest, infra::utils::error::AppError};

pub fn validate_request(req: &CreateTaskRequest) -> Result<(), AppError> {
    req.priority
        .as_ref()
        .map(|p| p.as_str())
        .filter(|p| !p.is_empty())
        .ok_or_else(|| AppError::BadRequest("Priority is required".to_string()))?;
    Ok(())
}
