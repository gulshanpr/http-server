use axum::routing::{get, Router};
use crate::handlers::health::{health_handler};
use crate::handlers::root::root_handler;
use crate::handlers::users::{list_user, users_handler};
use crate::state::AppState;

pub fn create_routes() -> Router<AppState> {
    Router::new().route("/health", get(health_handler))
        .route("/", get(root_handler))
        .route("/users/:id", get(users_handler))
        .route("/users", get(list_user))
}