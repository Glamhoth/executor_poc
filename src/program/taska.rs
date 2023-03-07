use core::fmt::Write;

use cortex_m_semihosting::hio;

use crate::rtos::messagequeue::MessageQueue;
use crate::rtos::safecell::SafeCell;
use crate::rtos::task::{Task, TaskState};
use crate::rtos::taskdata::TaskData;

pub struct TaskAData {
    pub counter: u32,
    pub data_queue: &'static MessageQueue<u32, 64>,
}

impl TaskData for TaskAData {}

pub fn task_a(data: &SafeCell<TaskAData>) {
    let mut stdout = hio::hstdout().unwrap();

    let counter = &mut data.as_ref_mut().counter;
    *counter += 1;

    write!(stdout, "TaskA: {}\n", counter);

    // match self.data_queue.dequeue() {
    //     Some(value) => {
    //         self.set_state(TaskState::Ready);
    //         write!(stdout, "TaskA! {};\n", value).unwrap();
    //     }
    //     None => {
    //         self.set_state(TaskState::Blocked);
    //         self.data_queue.block(MyTasks::TaskA(self));
    //     }
    // }
}
