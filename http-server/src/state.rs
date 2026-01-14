use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState{
    pub config: AppConfig
}

impl AppState {
    pub fn new() -> Self {
        Self {
            config: AppConfig::from_env()
        }
    }
}