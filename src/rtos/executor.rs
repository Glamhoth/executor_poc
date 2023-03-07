use core::ffi::c_void;
use core::any::Any;

use heapless::binary_heap::{BinaryHeap, Max};

use crate::rtos::task::TaskHandle;

pub struct Executor {
    task_queue: BinaryHeap<(TaskHandle, *const c_void), Max, 8>,
}

impl Executor {
    pub const fn new() -> Self {
        let task_queue = BinaryHeap::new();

        Executor { task_queue }
    }

    pub fn enqueue_task(&mut self, task: TaskHandle, queue: *const c_void) {

    }

    pub fn run(&self) {

    }
}
