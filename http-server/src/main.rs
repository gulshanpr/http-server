mod state;
mod handlers;
mod routes;
mod app;
mod types;
mod error;
mod config;

use tracing::info;
use tracing_subscriber::EnvFilter;
use crate::app::create_app;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let state = AppState::new();
    let address = format!("{}:{}", state.config.host, state.config.port);
    let app = create_app(state);


    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(ctrl_c())
        .await
        .unwrap();
}

async fn ctrl_c() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install ctrl c handler");

    info!("server shutting down");
}