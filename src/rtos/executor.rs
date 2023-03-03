use core::any::Any;

use heapless::binary_heap::{BinaryHeap, Max};

use crate::rtos::task::Task;

pub struct Executor<T>
where
    T: Any + Task + Ord,
{
    task_queue: BinaryHeap<T, Max, 8>,
}

impl<T> Executor<T>
where
    T: Any + Task + Ord
{
    pub const fn new() -> Self {
        let task_queue = BinaryHeap::new();

        Executor { task_queue }
    }

    pub fn enqueue_task(&mut self, task: T) {
        self.task_queue.push(task);
    }
}
