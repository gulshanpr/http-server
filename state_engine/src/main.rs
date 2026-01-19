use std::path::PathBuf;
use crate::domain::event::Event;
use crate::domain::job::Job;
use crate::domain::state::JobState;
use crate::storage::file::FileJobStore;
use crate::storage::JobStore;
use tokio::sync::mpsc;
use crate::engine::transition::TransitionResult;
use crate::runtime::worker::JobWorker;

mod engine;
mod domain;
mod storage;
mod runtime;

fn process_event(event: Event) {
    match event {
        Event::Start => {
            println!("event start");
        },
        Event::Finish => {
            println!("event end");
        },
        Event::Failed(reason) => {
            println!("event failed {}", reason);
        }
    };
}
#[tokio::main]
async fn main() {
    let job = Job::new(6);

    let (worker, sender) = JobWorker::new(job);

    let handle = tokio::spawn(worker.run());

    sender.send(Event::Finish).await.unwrap();
    sender.send(Event::Start).await.unwrap();
    sender.send(Event::Finish).await.unwrap();

    drop(sender);

    handle.await.unwrap();

}
