use crate::rtos::critcell::CritCell;
use crate::rtos::queue::Queue;
use crate::rtos::task::{Task, TaskHandle, TaskState};

#[derive(Debug)]
pub struct MessageQueue<E, const N: usize> {
    queue: CritCell<heapless::spsc::Queue<E, N>>,
    blocking: CritCell<heapless::spsc::Queue<TaskHandle, 8>>,
}

impl<E, const N: usize> Queue<E> for MessageQueue<E, N> {
    fn enqueue(&self, elem: E) -> Result<(), E> {
        self.queue.lock(|queue| queue.enqueue(elem))
    }

    fn dequeue(&self) -> Option<E> {
        self.queue.lock(|queue| queue.dequeue())
    }

    fn block(&self, task: &'static dyn Task) {
        // self.blocking
        //     .lock(|q| q.enqueue(TaskHandle(task)).expect("Blocking list full"));
    }

    fn notify(&self) {
        // self.blocking.lock(|q| match q.dequeue() {
        //     Some(task) => unsafe { (*task.0).set_state(TaskState::Ready) },
        //     None => (),
        // });
    }
}

impl<E, const N: usize> MessageQueue<E, N> {
    pub const fn new() -> Self {
        MessageQueue {
            queue: CritCell::new(heapless::spsc::Queue::new()),
            blocking: CritCell::new(heapless::spsc::Queue::new()),
        }
    }
}
