use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobState {
    Pending,
    Running,
    Failed(String),
    Completed,
}