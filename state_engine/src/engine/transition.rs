use crate::domain::event::Event;
use crate::domain::state::JobState;

#[derive(Debug)]
pub enum TransitionResult {
    Applied,
    Ignored,
}

impl JobState {
    pub fn apply(self, event: Event) -> (JobState, TransitionResult) {
        match (self, event) {
            (JobState::Pending, Event::Start) => (JobState::Running, TransitionResult::Applied),
            (JobState::Running, Event::Failed(reason)) => {
                (JobState::Failed(reason), TransitionResult::Applied)
            }
            (JobState::Running, Event::Finish) => (JobState::Completed, TransitionResult::Applied),

            // invalid state -> ignored
            (state, _) => (state, TransitionResult::Ignored),
        }
    }
}
