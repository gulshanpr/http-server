use crate::domain::job::Job;

pub mod file;

pub trait JobStore {
    fn save(&self, job: &Job) -> Result<(), String>;
    fn load(&self, id: u64) -> Result<Job, String>;
}