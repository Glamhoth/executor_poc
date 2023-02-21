use crate::rtos::tasklist::TaskList;

use cortex_m::asm::wfe;
use cortex_m::interrupt;
use heapless::binary_heap::{BinaryHeap, Max};

type TaskArray<T, const TASK_COUNT: usize> = [T; TASK_COUNT];
type TaskQueue<T, const TASK_COUNT: usize> = BinaryHeap<T, Max, TASK_COUNT>;

pub struct Executor<TL, const TASK_COUNT: usize>
where
    TL: TaskList + 'static,
{
    tasks: TaskQueue<TL, TASK_COUNT>,
}

impl<TL, const TASK_COUNT: usize> Executor<TL, TASK_COUNT>
where
    TL: TaskList + core::cmp::Ord + core::fmt::Debug,
{
    pub fn new(defined_tasks: TaskArray<TL, TASK_COUNT>) -> Self {
        let mut tasks = BinaryHeap::new();

        for task in defined_tasks {
            tasks.push(task).expect("Task queue full");
        }

        Executor { tasks }
    }

    pub fn run_next_task(&mut self) {
        loop {
            let next_task = interrupt::free(|_| {
                let task = self.tasks.peek_mut();
                task
            });

            match next_task {
                Some(mut task) => {
                    let task_state = task.dispatch();
                }
                None => (),
            }

            wfe();
        }
    }
}
