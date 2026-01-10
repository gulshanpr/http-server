use crate::state::AppState;
use axum::extract::{State, Json};
use crate::error::AppError;
use crate::types::responses::HealthResponse;

pub async fn health_handler(State(state): State<AppState>) -> Result<Json<HealthResponse>, AppError> {
    Ok(Json(HealthResponse {
        status: format!("ok - {}", state.config.app_name)
    }))
}