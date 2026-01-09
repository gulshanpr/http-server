#[derive(Clone)]
pub struct AppState{
    app_name: String
}

impl AppState {
    pub fn new() -> Self {
        Self {
            app_name: "Rust http server".to_string(),
        }
    }
}