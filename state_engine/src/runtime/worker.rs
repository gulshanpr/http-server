use crate::domain::event::Event;
use crate::domain::job::Job;
use crate::engine::transition::TransitionResult;
use crate::storage::JobStore;
use tokio::sync::mpsc;
use tracing::error;

pub struct JobWorker<S>
where
    S: JobStore,
{
    job: Job,
    store: S,
    receiver: mpsc::Receiver<Event>,
}

impl<S> JobWorker<S>
where
    S: JobStore,
{
    pub fn new(job: Job, store: S) -> (Self, mpsc::Sender<Event>) {
        let (tx, rx) = mpsc::channel(8);

        let worker = Self {
            job,
            store,
            receiver: rx,
        };

        (worker, tx)
    }

    pub async fn run(mut self) {
        loop {
            tokio::select! {
                Some(event) = self.receiver.recv() => {
                    let result = self.job.handle(event);

                    if let TransitionResult::Applied = result {
                        if let Err(e) = self.store.save(&self.job) {
                            error!(error = %e, "failed to persist job");
                        }
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    println!("shutdown signal recieved");
                    break;
                }
            }
        }
        println!("worker stopped, final state = {:?}", self.job.state());
    }
}
