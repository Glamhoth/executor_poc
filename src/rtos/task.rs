#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
}

pub trait Task {
    fn get_state(&self) -> TaskState;
    fn set_state(&self, state: TaskState);

    fn get_last_running_time(&self) -> u64;
    fn set_last_running_time(&self, time: u64);

    fn step(&'static self);
}

pub trait TaskList: Ord {
    fn get_state(&self) -> TaskState;
    fn set_state(&self, state: TaskState);

    fn get_last_running_time(&self) -> u64;
    fn set_last_running_time(&self, time: u64);

    fn dispatch(&self);
}
