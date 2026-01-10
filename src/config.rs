use std::env;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub app_name: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            host: env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("APP_PORT").ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3000),
            app_name: env::var("APP_NAME").unwrap_or_else(|_| "RUST Http server".to_string())
        }
    }
}