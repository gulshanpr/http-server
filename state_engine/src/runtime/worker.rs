use tokio::sync::mpsc;
use crate::domain::event::Event;
use crate::domain::job::Job;

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
            let run = self.job.handle(event);

            
        }
    }
}