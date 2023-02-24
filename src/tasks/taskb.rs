use core::fmt::Write;

use crate::rtos::task::{Task, TaskState};
use crate::rtos::cell::SafeCell;

use cortex_m_semihosting::hio;

#[derive(Debug)]
pub struct TaskB {
    state: TaskState,
    last_running_time: SafeCell<u64>,
}

impl TaskB {
    pub const fn new() -> Self {
        TaskB {
            state: TaskState::Ready,
            last_running_time: SafeCell::new(0),
        }
    }
}

impl Task for TaskB {
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
        stdout.write_str("Hello, TaskB!\n").unwrap();
        TaskState::Ready
    }
}
