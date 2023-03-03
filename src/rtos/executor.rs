use heapless::binary_heap::{BinaryHeap, Max};

type TaskQueue<const N: usize> = BinaryHeap<u32, Max, N>;

pub struct Executor<const N: usize> {
    task_queue: TaskQueue<N>,
}

impl<const N: usize> Executor<N> {
    pub const fn new() -> Self {
        let task_queue = BinaryHeap::new();

        Executor { task_queue }
    }
}
