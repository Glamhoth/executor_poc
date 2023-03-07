use crate::rtos::task::Task;

pub trait Queue<E> {
    fn enqueue(&self, elem: E) -> Result<(), E>;
    fn dequeue(&self) -> Option<E>;
    fn block(&self, task: &'static dyn Task);
    fn notify(&self);
}
