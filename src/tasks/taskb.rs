use crate::rtos::task::{Task, TaskState};

use cortex_m_semihosting::hprintln;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskB {
    state: TaskState,
    last_running_time: u64,
}

impl TaskB {
    pub fn new(state: TaskState) -> Self {
        TaskB { state, last_running_time: 0 }
    }
}

impl Task for TaskB {
    fn get_state(&self) -> &TaskState {
        &self.state
    }

    fn set_state(&mut self, state: TaskState) {
        self.state = state;
    }

    fn get_last_running_time(&self) -> u64 {
        self.last_running_time
    }

    fn set_last_running_time(&mut self, time: u64) {
        self.last_running_time = time;
    }

    fn step(&mut self) -> TaskState {
        hprintln!("Hello, TaskB!");
        TaskState::Ready
    }
}
