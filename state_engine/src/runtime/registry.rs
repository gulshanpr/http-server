use std::collections::HashMap;
use tokio::sync::mpsc;
use crate::domain::event::Event;
use crate::domain::job::Job;
use crate::runtime::worker::JobWorker;
use crate::storage::JobStore;

pub struct WorkerRegistry<S>
where
    S: JobStore + Clone + Send + Sync + 'static,
{
    workers: HashMap<u64, mpsc::Sender<Event>>,
    store: S
}
impl <S> WorkerRegistry<S>
where
    S: JobStore + Clone + Send + Sync + 'static
{
    pub async fn send(&mut self, job_id: u64, event: Event) {
        if let Some(sender) = self.workers.get(&job_id) {
            sender.send(event).await.ok();
            return;
        }

        let job = self.store.load(job_id).unwrap_or_else(|_| Job::new(job_id));

        let (worker, sender) = JobWorker::new(job, self.store.clone());

        tokio::spawn(worker.run());

        sender.send(event).await.ok();
        self.workers.insert(job_id, sender);
    }
}