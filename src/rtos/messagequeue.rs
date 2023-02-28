use crate::rtos::cell::SafeCell;
use crate::rtos::queue::Queue;
use crate::rtos::task::{TaskList, TaskState};

#[derive(Debug)]
pub struct MessageQueue<TL, E, const N: usize>
where
    TL: TaskList,
{
    queue: SafeCell<heapless::spsc::Queue<E, N>>,
    blocking: SafeCell<heapless::Vec<TL, 8>>,
}

impl<TL, E, const N: usize> Queue<E, TL> for MessageQueue<TL, E, N>
where
    TL: TaskList + core::fmt::Debug + 'static,
{
    fn enqueue(&self, elem: E) -> Result<(), E> {
        self.queue.lock(|queue| queue.enqueue(elem))
    }

    fn dequeue(&self) -> Option<E> {
        self.queue.lock(|queue| queue.dequeue())
    }

    fn block(&self, task: TL) {
        self.blocking
            .lock(|vec| vec.push(task).expect("Blocking list full"));
    }

    fn notify(&self) {
        self.blocking.lock(|vec| {
            for task in &mut *vec {
                task.set_state(TaskState::Ready);
            }
            vec.clear();
        });
    }
}

impl<TL, E, const N: usize> MessageQueue<TL, E, N>
where
    TL: TaskList,
{
    pub const fn new() -> Self {
        MessageQueue {
            queue: SafeCell::new(heapless::spsc::Queue::new()),
            blocking: SafeCell::new(heapless::Vec::new()),
        }
    }
}
