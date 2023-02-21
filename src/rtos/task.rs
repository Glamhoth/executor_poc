#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum TaskState {
    Ready,
    Running,
    Blocked
}


pub trait Task {
    fn get_state(&self) -> &TaskState;
    fn set_state(&mut self, state: TaskState);

    fn get_last_running_time(&self) -> u64;
    fn set_last_running_time(&mut self, time: u64);

    fn step(&mut self) -> TaskState;
}
