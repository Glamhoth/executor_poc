use core::ffi::c_void;
use heapless::spsc::Queue;

use crate::rtos::task::Task;

type StepFn<E> = fn(E);

pub struct Tasklet<E> {
    step_fn: StepFn<E>,
    data_queue: Queue<E, 8>,
}

impl<E> Tasklet<E> {
    pub const fn new(step_fn: StepFn<E>) -> Self {
        let data_queue = Queue::new();

        Tasklet { step_fn, data_queue }
    }
}

impl<E> Task for Tasklet<E> {
    fn step(&self) {
        (self.step_fn)(self.data_queue.dequeue());
    }
}

unsafe impl<E> Sync for Tasklet<E> where E: Send {}
