use heapless::binary_heap::{BinaryHeap, Max};

use crate::rtos::critcell::CritCell;
use crate::rtos::safecell::SafeCell;
use crate::rtos::task::{Task, TaskHandle, TaskState};

type TaskQueue<const N: usize> = BinaryHeap<TaskHandle, Max, N>;
type TaskList<const N: usize> = [*const dyn Task; N];

pub struct Executor<const N: usize> {
    system_time: u64,
    task_list: TaskList<N>,
    task_queue: TaskQueue<N>,
}

impl<const N: usize> Executor<N> {
    pub const fn new(task_list: TaskList<N>) -> Self {
        let task_queue = BinaryHeap::new();

        Executor {
            system_time: 0,
            task_list,
            task_queue,
        }
    }

    fn update_system_time(&mut self) {
        self.system_time += 1;
    }

    pub fn enqueue_task(&mut self, task: *const dyn Task) {
        unsafe { (*task).set_state(TaskState::Ready) };

        let task_handle = TaskHandle {
            task,
            enqueue_time: self.system_time,
        };
        self.task_queue.push(task_handle).expect("Task queue full");
    }

    fn update_tasks(&mut self) {
        for task in self.task_list {
            let task_state = unsafe { (*task).get_state() };
            let task_has_data = unsafe { (*task).has_data() };

            if task_state == TaskState::Waiting && task_has_data {
                self.enqueue_task(task);
            }
        }
    }

    pub fn start(&mut self) -> ! {
        self.update_tasks();

        loop {
            self.update_system_time();

            let next_task = self.task_queue.pop();

            match next_task {
                Some(ready_task) => unsafe {
                    (*ready_task.task).step();
                },
                None => (),
            }

            self.update_tasks();
        }
    }
}
