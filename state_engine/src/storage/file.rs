use std::fs;
use std::path::PathBuf;
use crate::domain::job::Job;
use crate::storage::JobStore;
use serde_json;

pub struct FileJobStore {
    base_path: PathBuf
}

impl FileJobStore {
    pub fn new(base_path: PathBuf) -> Self {
        Self {base_path}
    }

    fn join_path(&self, id: u64) -> PathBuf {
        self.base_path.join(format!("job_{}.json", id))
    }
}

impl JobStore for FileJobStore {
    fn save(&self, job: &Job) -> Result<(), String> {
        let path = self.join_path(job.id());

        let data = serde_json::to_string_pretty(job)
            .map_err(|err| err.to_string())?;

        fs::write(path, data)
            .map_err(|err| err.to_string())?;

        Ok(())
    }

    fn load(&self, id: u64) -> Result<Job, String> {
        let path = self.join_path(id);

        let data = fs::read_to_string(path)
            .map_err(|err| err.to_string())?;

        let job = serde_json::from_str(&data)
            .map_err(|err| err.to_string())?;

        Ok(job)

    }
}