use crate::rtos::task::{Task, TaskState};

use cortex_m_semihosting::hprintln;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskB {
    state: TaskState,
}

impl TaskB {
    pub fn new(state: TaskState) -> Self {
        TaskB { state }
    }
}

impl Task for TaskB {
    fn state(&self) -> &TaskState {
        &self.state
    }

    fn step(&mut self) -> TaskState {
        hprintln!("Hello, TaskB!");
        TaskState::Ready
    }
}
