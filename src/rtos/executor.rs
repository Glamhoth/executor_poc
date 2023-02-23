use crate::rtos::cell::ThinCell;
use crate::rtos::task::TaskState;
use crate::rtos::tasklist::TaskList;

use heapless::binary_heap::{BinaryHeap, Max};

type TaskArray<T, const TASK_COUNT: usize> = [T; TASK_COUNT];
type TaskQueue<T, const TASK_COUNT: usize> = BinaryHeap<T, Max, TASK_COUNT>;

pub struct Executor<TL, const TASK_COUNT: usize>
where
    TL: TaskList + 'static,
{
    system_time: &'static u64,
    task_queue: TaskQueue<&'static ThinCell<TL>, TASK_COUNT>,
}

impl<TL, const TASK_COUNT: usize> Executor<TL, TASK_COUNT>
where
    TL: TaskList + core::cmp::Ord + core::fmt::Debug,
{
    pub fn new(system_time: &'static u64) -> Self {
        let task_queue = BinaryHeap::new();

        Executor {
            system_time,
            task_queue,
        }
    }

    pub fn register_task(&mut self, task: &'static ThinCell<TL>) {
        self.task_queue.push(task).expect("Task queue is full");
    }

    pub fn run_next_task(&mut self) {
        loop {
            let current_time = { *self.system_time };

            let next_task = self.task_queue.pop();

            match next_task {
                Some(task) => {
                    let mut ready_task = unsafe { task.as_ref_mut() };

                    if *ready_task.get_state() == TaskState::Ready {
                        ready_task.set_last_running_time(current_time);

                        let task_state = ready_task.dispatch();
                        ready_task.set_state(task_state);
                    }

                    self.task_queue.push(task);
                }
                None => (),
            }
        }
    }
}
