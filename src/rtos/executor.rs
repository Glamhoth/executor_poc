use crate::rtos::cell::SafeCell;
use crate::rtos::task::{TaskList, TaskState};

use cortex_m_semihosting::hprintln;
use heapless::binary_heap::{BinaryHeap, Max};

type TaskArray<T, const TASK_COUNT: usize> = [T; TASK_COUNT];
type TaskQueue<T, const TASK_COUNT: usize> = BinaryHeap<T, Max, TASK_COUNT>;

pub struct Executor<TL, const TASK_COUNT: usize>
where
    TL: TaskList + 'static,
{
    system_time: SafeCell<u64>,
    task_queue: SafeCell<TaskQueue<&'static TL, TASK_COUNT>>,
}

impl<TL, const TASK_COUNT: usize> Executor<TL, TASK_COUNT>
where
    TL: TaskList + core::cmp::Ord + core::fmt::Debug,
{
    pub const fn new() -> Self {
        Executor {
            system_time: SafeCell::new(0),
            task_queue: SafeCell::new(BinaryHeap::new()),
        }
    }

    pub fn enqueue_task(&self, task: &'static TL) {
        self.task_queue
            .lock(|queue| queue.push(task))
            .expect("Task queue is full");
    }

    pub fn update_system_time(&self) {
        self.system_time.lock(|time| {
            *time += 1;
        });
    }

    pub fn start(&self) {
        loop {
            self.update_system_time();

            let next_task = self.task_queue.lock(|queue| queue.pop());

            match next_task {
                Some(ready_task) => {
                    let current_time = self.system_time.lock(|time| *time);
                    ready_task.set_last_running_time(current_time);

                    let task_state = ready_task.dispatch();

                    if (task_state == TaskState::Ready) {
                        self.task_queue
                            .lock(|queue| queue.push(ready_task))
                            .expect("Task queue is full");
                    }
                }
                None => (),
            }
        }
    }
}
