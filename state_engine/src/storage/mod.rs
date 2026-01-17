use crate::domain::job::Job;

pub mod file;

// anyone can store the job, but need to have these properties
pub trait JobStore {
    fn save(&self, job: &Job) -> Result<(), String>;
    fn load(&self, id: u64) -> Result<Job, String>;
}
