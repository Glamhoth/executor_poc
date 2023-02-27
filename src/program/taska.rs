use core::fmt::Write;

use cortex_m_semihosting::hio;

use crate::rtos::task::{Task, TaskState};
use crate::rtos::cell::SafeCell;
use crate::rtos::queue::Queue;
use crate::rtos::messagequeue::MessageQueue;

#[derive(Debug)]
pub struct TaskA {
    state: TaskState,
    last_running_time: SafeCell<u64>,
    data_queue: &'static MessageQueue<u32, 64>
}

impl TaskA {
    pub const fn new(data_queue: &'static MessageQueue<u32, 64>) -> Self {
        TaskA {
            state: TaskState::Ready,
            last_running_time: SafeCell::new(0),
            data_queue
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

        match self.data_queue.dequeue() {
            Some(value) => write!(stdout, "TaskA! {}\n", value).unwrap(),
            None => return TaskState::Blocked
        }

        TaskState::Ready
    }
}
