use tokio::sync::mpsc;
use crate::domain::event::Event;
use crate::domain::job::Job;
use crate::engine::transition::TransitionResult;

pub struct JobWorker {
    job: Job,
    receiver: mpsc::Receiver<Event>,
}

impl JobWorker {
    pub fn new(job: Job) -> (Self, mpsc::Sender<Event>) {
        let (tx, rx) = mpsc::channel(8);

        let worker = Self {
            job,
            receiver: rx,
        };

        (worker, tx)
    }

    pub async fn run(mut self) {
        while let Some(event) = self.receiver.recv().await {
            let result = self.job.handle(event);

            match result {
                TransitionResult::Applied => {
                    println!("state changed -> {:?}", self.job.state());
                }

                TransitionResult::Ignored => {
                    println!("event ignored, state = {:?}", self.job.state());
                }
            }

            println!("worker stopped, final state = {:?}", self.job.state());

        }
    }
}