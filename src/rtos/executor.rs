use cortex_m_semihosting::hprintln;
use heapless::binary_heap::{BinaryHeap, Max};

use crate::rtos::task::{Task, TaskHandle, TaskState};

type TaskQueue<const TASK_COUNT: usize> = BinaryHeap<TaskHandle, Max, TASK_COUNT>;

pub struct Executor<const TASK_COUNT: usize> {
    system_time: u64,
    task_queue: TaskQueue<TASK_COUNT>,
}

impl<const TASK_COUNT: usize> Executor<TASK_COUNT> {
    pub const fn new() -> Self {
        Executor {
            system_time: 0,
            task_queue: BinaryHeap::new(),
        }
    }

    pub fn enqueue_task(&mut self, task: &'static dyn Task) {
        self.task_queue.push(TaskHandle(task));
    }

    pub fn update_system_time(&mut self) {
        self.system_time += 1;
    }

    pub fn start(&mut self) {
        loop {
            self.update_system_time();

            let next_task = self.task_queue.pop();

            match next_task {
                Some(ready_task) => {
                    let current_time = self.system_time;

                    unsafe {
                        (*ready_task.0).set_last_running_time(current_time);
                        (*ready_task.0).step();
                    }
                }
                None => (),
            }
        }
    }
}
