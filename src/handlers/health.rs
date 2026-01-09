use crate::state::AppState;
use axum::extract::{State, Json};
use crate::types::responses::HealthResponse;

pub async fn health_handler(State(_state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}