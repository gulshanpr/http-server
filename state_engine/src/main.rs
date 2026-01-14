use crate::domain::event::Event;
use crate::domain::job::Job;
use crate::domain::state::JobState;

mod engine;
mod domain;

fn main() {

    let mut job = Job::new(1);
    println!("job created pending job state {:?}", job.state());

    job.handle(Event::Failed("testing".to_string()));
    println!("job failed before starting(invalid state) {:?}", job.state());

    job.handle(Event::Start);
    println!("job started {:?}", job.state());


    job.handle(Event::Finish);
    println!("job completed {:?}", job.state());

}
