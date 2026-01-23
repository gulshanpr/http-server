#[derive(Debug)]
pub enum Event {
    Start,
    Finish,
    Failed(String),
}
