use crate::rtos::task::{Task, TaskState};

use cortex_m_semihosting::hprintln;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskA {
    state: TaskState,
}

impl TaskA {
    pub fn new(state: TaskState) -> Self {
        TaskA { state }
    }
}

impl Task for TaskA {
    fn state(&self) -> &TaskState {
        &self.state
    }

    fn step(&mut self) -> TaskState {
        hprintln!("Hello, TaskA!").unwrap();
        TaskState::Blocked
    }
}
