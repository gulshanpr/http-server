use crate::domain::event::Event;
use crate::domain::state::JobState;
use crate::engine::transition::TransitionResult;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    id: u64,
    job_state: JobState,
}

impl Job {
    pub fn new(id: u64) -> Self {
        Job {
            id,
            job_state: JobState::Pending,
        }
    }

    pub fn state(&self) -> &JobState {
        &self.job_state
    }

    pub fn handle(&mut self, event: Event) -> TransitionResult {
        let (new_state, result) = self.job_state.clone().apply(event);

        if let TransitionResult::Applied = result {
            self.job_state = new_state;
        }

        result
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}
