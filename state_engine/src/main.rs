use std::path::PathBuf;
use crate::domain::event::Event;
use crate::domain::job::Job;
use crate::domain::state::JobState;
use crate::storage::file::FileJobStore;
use crate::storage::JobStore;

mod engine;
mod domain;
mod storage;

fn main() {

    // location of the store
    let store = FileJobStore::new(PathBuf::from("./data"));

    // create dir.
    std::fs::create_dir_all("./data").unwrap();

    // mut because we want to change the job status
    let mut job = match store.load(1) {
        Ok(job) => {
            println!("job recovered from disk");
            job
        },
        Err(_) => {
            println!("Creating new job");
            Job::new(1)
        }
    };

    job.handle(Event::Start);
    store.save(&job).unwrap();

    println!("current job state {:?}", job.state());

}
