use crate::rtos::cell::SafeCell;
use crate::rtos::task::{TaskList, TaskState};

use cortex_m_semihosting::hprintln;
use heapless::binary_heap::{BinaryHeap, Max};

type TaskArray<T, const TASK_COUNT: usize> = [T; TASK_COUNT];
type TaskQueue<T, const TASK_COUNT: usize> = BinaryHeap<T, Max, TASK_COUNT>;

pub struct Executor<TL, const TASK_COUNT: usize>
where
    TL: TaskList,
{
    system_time: u64,
    task_queue: TaskQueue<TL, TASK_COUNT>,
}

impl<TL, const TASK_COUNT: usize> Executor<TL, TASK_COUNT>
where
    TL: TaskList + core::cmp::Ord + core::fmt::Debug,
{
    pub const fn new() -> Self {
        Executor {
            system_time: 0,
            task_queue: BinaryHeap::new(),
        }
    }

    pub fn enqueue_task(&mut self, task: TL) {
        self.task_queue.push(task);
    }

    pub fn update_system_time(&mut self) {
        self.system_time += 1;
    }

    pub fn start(&mut self) {
        loop {
            self.update_system_time();

            let next_task = self.task_queue.peek_mut();

            match next_task {
                Some(ready_task) => {
                    if ready_task.get_state() == TaskState::Ready {
                        let current_time = self.system_time;
                        ready_task.set_last_running_time(current_time);

                        ready_task.dispatch();
                    }
                }
                None => (),
            }
        }
    }
}
