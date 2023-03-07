use core::fmt::Debug;

use heapless::spsc::Queue;

use crate::rtos::critcell::CritCell;
use crate::rtos::notifiable::Notifiable;
use crate::rtos::safecell::SafeCell;
use crate::rtos::task::Task;

type StepFn<T, E> = fn(&'static SafeCell<T>, E);

pub struct Tasklet<T, E>
where
    T: 'static,
{
    local_data: &'static SafeCell<T>,
    data_queue: CritCell<Queue<E, 8>>,
    priority: u8,
    step_fn: StepFn<T, E>,
}

impl<T, E> Tasklet<T, E>
where
    T: 'static,
{
    pub const fn new(local_data: &'static SafeCell<T>, step_fn: StepFn<T, E>, priority: u8) -> Self {
        let data_queue = CritCell::new(Queue::new());

        Tasklet {
            local_data,
            data_queue,
            priority,
            step_fn,
        }
    }
}

impl<T, E> Task for Tasklet<T, E> {
    fn get_priority(&self) -> u8 {
        self.priority
    }

    fn step(&self) {
        let data = self.data_queue.lock(|q| q.dequeue());

        match data {
            Some(d) => (self.step_fn)(self.local_data, d),
            None => (),
        };
    }
}

impl<T, E> Notifiable<E> for Tasklet<T, E>
where
    E: Debug,
{
    fn notify(&self, data: E) {
        self.data_queue
            .lock(|q| q.enqueue(data).expect("Data queue full"));
    }
}
