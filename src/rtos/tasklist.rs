use crate::rtos::task::TaskState;

pub trait TaskList: Ord {
    fn get_state(&self) -> &TaskState;
    fn set_state(&mut self, state: TaskState);

    fn get_last_running_time(&self) -> u64;
    fn set_last_running_time(&mut self, time: u64);

    fn dispatch(&mut self) -> TaskState;
}

