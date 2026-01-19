use std::path::PathBuf;
use crate::domain::event::Event;
use crate::domain::job::Job;
use crate::domain::state::JobState;
use crate::storage::file::FileJobStore;
use crate::storage::JobStore;
use tokio::sync::mpsc;
use crate::engine::transition::TransitionResult;

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
    let (tx, mut rx) = mpsc::channel::<Event>(8);

    let mut job = Job::new(3);

    tx.send(Event::Failed("network error".to_string())).await.unwrap();
    tx.send(Event::Start).await.unwrap();
    tx.send(Event::Finish).await.unwrap();

    drop(tx);

    while let Some(event) = rx.recv().await {
        let result = job.handle(event);

        match result {
            TransitionResult::Applied => {
                println!("state changes -> {:?}", job.state());
            }
            TransitionResult::Ignored => {
                println!("state change ignored, state = {:?}", job.state());
            }
        };

    }

    println!("final state = {:?}", job.state());

}
