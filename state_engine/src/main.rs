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

    let store = FileJobStore::new(PathBuf::from("./data"));

    std::fs::create_dir_all("./data").unwrap();

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
