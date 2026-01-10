use axum::http::Method;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use crate::routes::create_routes;
use crate::state::AppState;

pub fn create_app(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods([Method::GET]) // GET for now
        .allow_origin(Any);
    Router::new().merge(create_routes()).with_state(state).layer(cors)
}