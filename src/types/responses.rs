use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct Pagination {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}