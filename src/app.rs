use axum::Router;
use crate::routes::create_routes;
use crate::state::AppState;

pub fn create_app(state: AppState) -> Router {
    Router::new().merge(create_routes()).with_state(state)
}