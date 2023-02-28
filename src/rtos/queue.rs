use crate::rtos::task::TaskList;

pub trait Queue<E, TL>
where
    TL: TaskList
{
    fn enqueue(&self, elem: E) -> Result<(), E>;
    fn dequeue(&self) -> Option<E>;
    fn block(&self, task: TL);
    fn notify(&self);
}
