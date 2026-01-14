use crate::domain::event::Event;
use crate::domain::state::JobState;

#[derive(Debug)]
pub struct Job {
    id: u64,
    job_state: JobState,
}

impl Job {
    pub fn new(id: u64) -> Self {
        Job {
            id,
            job_state: JobState::Pending
        }
    }

    pub fn state(&self) -> &JobState {
        &self.job_state
    }

    pub fn handle(&mut self, event: Event) {
        let new_state = self.job_state.clone().apply(event);

        self.job_state = new_state;
    }
}