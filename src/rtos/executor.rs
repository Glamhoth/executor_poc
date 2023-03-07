use heapless::binary_heap::{BinaryHeap, Max};

use crate::rtos::critcell::CritCell;
use crate::rtos::safecell::SafeCell;
use crate::rtos::task::{Task, TaskHandle};

type TaskQueue = BinaryHeap<TaskHandle, Max, 8>;

pub struct Executor {
    system_time: SafeCell<u64>,
    task_queue: CritCell<TaskQueue>,
}

impl Executor {
    pub const fn new() -> Self {
        let task_queue = CritCell::new(BinaryHeap::new());

        Executor {
            system_time: SafeCell::new(0),
            task_queue,
        }
    }

    fn update_system_time(&self) {
        *self.system_time.as_ref_mut() += 1;
    }

    pub fn enqueue_task(&self, task: *const dyn Task) {
        self.task_queue.lock(|q| {
            let task_handle = TaskHandle(task, *self.system_time.as_ref());
            q.push(task_handle).expect("Task queue full");
        });
    }

    pub fn start(&self) -> ! {
        loop {
            self.update_system_time();

            let next_task = self.task_queue.lock(|q| q.pop());

            match next_task {
                Some(ready_task) => {
                    unsafe {
                        (*ready_task.0).step();
                    }
                }
                None => ()
            }
        }
    }
}
