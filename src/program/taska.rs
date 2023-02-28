use core::fmt::Write;

use cortex_m_semihosting::hio;

use crate::rtos::cell::SafeCell;
use crate::rtos::messagequeue::MessageQueue;
use crate::rtos::queue::Queue;
use crate::rtos::task::{Task, TaskState};
use crate::MyTasks;

#[derive(Debug)]
pub struct TaskA {
    state: SafeCell<TaskState>,
    last_running_time: SafeCell<u64>,
    data_queue: &'static MessageQueue<MyTasks, u32, 64>,
}

impl TaskA {
    pub const fn new(data_queue: &'static MessageQueue<MyTasks, u32, 64>) -> Self {
        TaskA {
            state: SafeCell::new(TaskState::Ready),
            last_running_time: SafeCell::new(0),
            data_queue,
        }
    }
}

impl Task for TaskA {
    fn get_state(&self) -> TaskState {
        self.state.lock(|s| *s)
    }

    fn set_state(&self, state: TaskState) {
        self.state.lock(|s| *s = state)
    }

    fn get_last_running_time(&self) -> u64 {
        self.last_running_time.lock(|t| *t)
    }

    fn set_last_running_time(&self, time: u64) {
        self.last_running_time.lock(|t| *t = time)
    }

    fn step(&'static self) {
        let mut stdout = hio::hstdout().unwrap();

        // write!(stdout, "TaskA\n");

        match self.data_queue.dequeue() {
            Some(value) => {
                self.set_state(TaskState::Ready);
                write!(stdout, "TaskA! {};\n", value).unwrap();
            }
            None => {
                self.set_state(TaskState::Blocked);
                self.data_queue.block(MyTasks::TaskA(self));
            }
        }
    }
}
