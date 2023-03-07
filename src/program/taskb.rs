use core::fmt::Write;

use cortex_m_semihosting::hio;

use crate::rtos::messagequeue::MessageQueue;
use crate::rtos::safecell::SafeCell;
use crate::rtos::task::{Task, TaskState};
use crate::rtos::taskdata::TaskData;

pub struct TaskBData {
    pub small_counter: u8,
    pub data_queue: &'static MessageQueue<u32, 64>,
}

impl TaskData for TaskBData {}

pub fn task_b(data: &SafeCell<TaskBData>) {
    let mut stdout = hio::hstdout().unwrap();

    let mut counter = &mut data.as_ref_mut().small_counter;
    *counter = (*counter).wrapping_add(1);

    write!(stdout, "TaskB: {}\n", counter);

    // match data_queue.dequeue() {
    //     Some(value) => {
    //         write!(stdout, "TaskB: {}; {}\n", counter, value).unwrap();
    //         TaskState::Ready
    //     }
    //     None => {
    //         data_queue.block(tasklet);
    //         TaskState::Blocked
    //     }
    // }
}
