use crate::domain::event::Event;
use crate::domain::state::JobState;

impl JobState {
    pub fn apply(self, event: Event) -> JobState {
        match (self, event) {
            (JobState::Pending, Event::Start) => JobState::Running,
            (JobState::Running, Event::Failed(reason)) => JobState::Failed(reason),
            (JobState::Running, Event::Finish) => JobState::Completed,

            (state, _) => state
        }
    }
}