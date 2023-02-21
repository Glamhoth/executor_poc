#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum TaskState {
    Ready,
    Running,
    Blocked
}


pub trait Task {
    fn state(&self) -> &TaskState;
    fn step(&mut self) -> TaskState;
}
