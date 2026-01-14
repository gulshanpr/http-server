#[derive(Debug, Clone)]
pub enum JobState {
    Pending,
    Running,
    Failed(String),
    Completed,
}