use axum::{Json, Router, extract::{Path, Query}, routing::get};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    up_time: u64,
}

#[derive(Deserialize)]
struct Pagination {
    page: Option<u64>,
    limit: Option<u64>
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .route("/users/:id", get(users_handler))
        .route("/users", get(list_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3004")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> &'static str {
    "hello world from the rust http server"
}

async fn health_handler() -> Json<HealthResponse> {
    let res = HealthResponse {
        status: "Active".to_string(),
        up_time: 100,
    };

    Json(res)
}

async fn users_handler(Path(user_id): Path<u64>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
            "user_id": user_id,
            "message": "user fetched successfully",
    }))
}

async fn list_user(Query(pagination): Query<Pagination>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "page": pagination.page.unwrap_or(1),
        "limit": pagination.limit.unwrap_or(10)
    }))
}