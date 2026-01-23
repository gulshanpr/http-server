use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobState {
    Pending,
    Running,
    Failed(String),
    Completed,
}
