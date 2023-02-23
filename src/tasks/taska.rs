use core::fmt::Write;

use cortex_m_semihosting::hio;

use crate::rtos::task::{Task, TaskState};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskA {
    state: TaskState,
    last_running_time: u64,
}

impl TaskA {
    pub fn new(state: TaskState) -> Self {
        TaskA {
            state,
            last_running_time: 0,
        }
    }
}

impl Task for TaskA {
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
        let mut stdout = hio::hstdout().unwrap();
        stdout.write_str("Hello, TaskA!\n").unwrap();
        TaskState::Ready
    }
}
