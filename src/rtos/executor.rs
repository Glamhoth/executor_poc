use crate::rtos::task::TaskState;
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
    system_time: &'static u64,
}

impl<TL, const TASK_COUNT: usize> Executor<TL, TASK_COUNT>
where
    TL: TaskList + core::cmp::Ord + core::fmt::Debug,
{
    pub fn new(defined_tasks: TaskArray<TL, TASK_COUNT>, system_time: &'static u64) -> Self {
        let mut tasks = BinaryHeap::new();

        for task in defined_tasks {
            tasks.push(task).expect("Task queue full");
        }

        Executor { tasks, system_time }
    }

    pub fn run_next_task(&mut self) {
        loop {
            let current_time = { *self.system_time };

            let next_task = interrupt::free(|_| {
                let task = self.tasks.peek_mut();
                task
            });

            match next_task {
                Some(mut task) => {
                    // task.set_state(TaskState::Running);
                    // task.set_last_running_time(current_time);

                    let task_state = task.dispatch();
                    // task.set_state(task_state);
                }
                None => (),
            }

            wfe();
        }
    }
}
