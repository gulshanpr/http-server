use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum AppError {
    NotFound,
    BadRequest(String),
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, "Not Found").into_response()
            }
            AppError::BadRequest(error) => {
                (StatusCode::BAD_REQUEST, error).into_response()
            }
            AppError::InternalError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        }
    }
}