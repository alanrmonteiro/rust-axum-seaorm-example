use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not Found")]
    NotFound(String),
    #[error("Internal Server Error: {0}")]
    InternalError(String),
    #[error("Bad Request: {0}")]
    BadRequest(String),
    //#[error("Database error")]
    //DatabaseError(#[from] sqlx::Error), // Example of wrapping external errors
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::InternalError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("An internal error occurred: {}", msg),
            ),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        (status, error_message).into_response()
    }
}
