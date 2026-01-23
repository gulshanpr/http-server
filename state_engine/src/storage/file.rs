use crate::domain::job::Job;
use crate::storage::JobStore;
use serde_json;
use std::fs;
use std::path::PathBuf;

pub struct FileJobStore {
    base_path: PathBuf,
}

impl FileJobStore {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    fn join_path(&self, id: u64) -> PathBuf {
        self.base_path.join(format!("job_{}.json", id))
    }
}

// implementing JobStore trait for FileJobStore that is for file storage
impl JobStore for FileJobStore {
    fn save(&self, job: &Job) -> Result<(), String> {
        // get the path
        let path = self.join_path(job.id());

        // struct to file compatible string format using serde_json (serializing)
        let data = serde_json::to_string_pretty(job).map_err(|err| err.to_string())?;

        // only we can write bytes or string not XYZ { } rust struct
        fs::write(path, data).map_err(|err| err.to_string())?;

        Ok(())
    }

    fn load(&self, id: u64) -> Result<Job, String> {
        // get the path
        let path = self.join_path(id);

        // read as a string from file
        let data = fs::read_to_string(path).map_err(|err| err.to_string())?;

        // Deserialize the string for struct
        let job = serde_json::from_str(&data).map_err(|err| err.to_string())?;

        Ok(job)
    }
}
