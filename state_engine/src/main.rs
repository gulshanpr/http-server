use crate::domain::event::Event;
use crate::domain::job::Job;
use crate::domain::state::JobState;
use crate::engine::transition::TransitionResult;
use crate::runtime::worker::JobWorker;
use crate::storage::JobStore;
use crate::storage::file::FileJobStore;
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;
use tracing_subscriber::FmtSubscriber;

mod domain;
mod engine;
mod runtime;
mod storage;

fn process_event(event: Event) {
    match event {
        Event::Start => {
            println!("event start");
        }
        Event::Finish => {
            println!("event end");
        }
        Event::Failed(reason) => {
            println!("event failed {}", reason);
        }
    };
}
#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    
    let store = FileJobStore::new(PathBuf::from("./data"));
    std::fs::create_dir_all("./data").unwrap();

    let job = store.load(1).unwrap_or_else(|_| Job::new(1));

    let (worker, sender) = JobWorker::new(job, store);

    let handle = tokio::spawn(worker.run());

    sender.send(Event::Finish).await.unwrap();
    sender.send(Event::Start).await.unwrap();
    sender.send(Event::Finish).await.unwrap();

    drop(sender);

    handle.await.unwrap();
}
