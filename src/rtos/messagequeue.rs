use crate::rtos::cell::SafeCell;
use crate::rtos::queue::Queue;

#[derive(Debug)]
pub struct MessageQueue<T, const N: usize> {
    queue: SafeCell<heapless::spsc::Queue<T, N>>,
}

impl<T, const N: usize> Queue<T> for MessageQueue<T, N>
where
    T: core::fmt::Debug,
{
    fn enqueue(&self, elem: T) -> Result<(), T> {
        self.queue.lock(|queue| queue.enqueue(elem))
    }

    fn dequeue(&self) -> Option<T> {
        self.queue.lock(|queue| queue.dequeue())
    }
}

impl<T, const N: usize> MessageQueue<T, N> {
    pub const fn new() -> Self {
        MessageQueue {
            queue: SafeCell::new(heapless::spsc::Queue::new()),
        }
    }
}
