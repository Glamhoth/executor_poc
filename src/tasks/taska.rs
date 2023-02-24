use core::fmt::Write;

use cortex_m_semihosting::hio;

use crate::rtos::task::{Task, TaskState};
use crate::rtos::cell::SafeCell;

#[derive(Debug)]
pub struct TaskA {
    state: TaskState,
    last_running_time: SafeCell<u64>,
}

impl TaskA {
    pub const fn new() -> Self {
        TaskA {
            state: TaskState::Ready,
            last_running_time: SafeCell::new(0),
        }
    }
}

impl Task for TaskA {
    // fn get_state(&self) -> &TaskState {
    //     &self.state
    // }

    // fn set_state(&mut self, state: TaskState) {
    //     self.state = state;
    // }

    fn get_last_running_time(&self) -> u64 {
        self.last_running_time.lock(|t| *t)
    }

    fn set_last_running_time(&self, time: u64) {
        self.last_running_time.lock(|t| *t = time)
    }

    fn step(&self) -> TaskState {
        let mut stdout = hio::hstdout().unwrap();
        stdout.write_str("Hello, TaskA!\n").unwrap();
        TaskState::Ready
    }
}
