use axum::extract::{Path, Query};
use axum::Json;
use crate::types::responses::Pagination;

pub async fn users_handler(Path(user_id): Path<u64>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "user_id": user_id,
        "message": "user fetched successfully",
    }))
}

pub async fn list_user(Query(pagination): Query<Pagination>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "page": pagination.page.unwrap_or(1),
        "limit": pagination.limit.unwrap_or(10)
    }))
}
