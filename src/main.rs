mod state;
mod handlers;
mod routes;
mod app;
mod types;

use crate::app::create_app;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    let state = AppState::new();
    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3004")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}