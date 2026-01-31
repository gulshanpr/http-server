use crate::domain::event::Event;
use crate::domain::job::Job;
use crate::domain::state::JobState;
use crate::engine::transition::TransitionResult;
use crate::runtime::worker::JobWorker;
use crate::storage::JobStore;
use crate::storage::file::FileJobStore;
use axum::Router;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{Route, post};
use std::path::{Path, PathBuf};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tracing_subscriber::FmtSubscriber;

mod domain;
mod engine;
mod runtime;
mod storage;

#[derive(Clone)]
struct AppState {
    sender: mpsc::Sender<Event>,
}

async fn start_job(State(state): State<AppState>) -> impl IntoResponse {
    if let Err(_) = state.sender.send(Event::Start).await {
        return "worker not running";
    }

    "start event sent"
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let store = FileJobStore::new(PathBuf::from("./data"));
    std::fs::create_dir_all("./data").unwrap();

    let job = store.load(5).unwrap_or_else(|_| Job::new(5));

    let (worker, sender) = JobWorker::new(job, store);
    let app_state = AppState { sender };

    let app = Router::new()
        .route("/start", post(start_job))
        .with_state(app_state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // let handle = tokio::spawn(worker.run());
    //
    // sender.send(Event::Finish).await.unwrap();
    // sender.send(Event::Start).await.unwrap();
    // sender.send(Event::Finish).await.unwrap();
    //
    // drop(sender);

    // handle.await.unwrap();
}
