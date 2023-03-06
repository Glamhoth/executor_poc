use heapless::binary_heap::{BinaryHeap, Max};

use super::task::Task;

type TaskQueue<const N: usize> = BinaryHeap<*const dyn Task, Max, N>;

pub struct Executor<const N: usize> {
    system_time: u64,
    task_queue: TaskQueue<N>,
}

impl<const N: usize> Executor<N> {
    pub const fn new() -> Self {
        let task_queue = BinaryHeap::new();

        Executor {
            system_time: 0,
            task_queue,
        }
    }

    pub fn enqueue_task(&mut self, task: &'static dyn Task) {
        self.task_queue.push(task);
    }

    pub fn update_system_time(&mut self) {
        self.system_time += 1;
    }
}
