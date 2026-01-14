use axum::extract::{Path, Query};
use axum::Json;
use tracing::{info, warn};
use crate::error::AppError;
use crate::types::responses::Pagination;

pub async fn users_handler(Path(user_id): Path<u64>) -> Result<Json<serde_json::Value>, AppError> {
    info!("fetching user with id {}", user_id);
    if user_id == 0 {
        warn!("zero user_id query {}", user_id);
        return Err(
            AppError::BadRequest(
            "user needs to be greater than zero".to_string()
        ))
    }
    Ok(
        Json(serde_json::json!({
        "user_id": user_id,
        "message": "user fetched successfully",
    }))
    )
}

pub async fn list_user(Query(pagination): Query<Pagination>) -> Result<Json<serde_json::Value>, AppError> {
    if let Some(limit) = pagination.limit {
        if limit == 0 {
            return Err(AppError::BadRequest(
                "limit must be greater than zero".to_string()
            ))
        }
    }

    if let Some(page) = pagination.page {
        if page == 0 {
            return Err(
                AppError::BadRequest(
                    "page must be greater than zero".to_string()
                )
            )
        }
    }
    Ok(
        Json(serde_json::json!({
        "page": pagination.page.unwrap_or(1),
        "limit": pagination.limit.unwrap_or(10)
    }))
    )
}
