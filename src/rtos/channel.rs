use crate::rtos::executor::Executor;
use crate::rtos::notifiable::{NotifHandle, Notifiable};
use crate::rtos::safecell::SafeCell;

use heapless::Vec;

pub struct Channel<E>
where
    E: Clone,
{
    // executor: &'static Executor,
    registered: SafeCell<Vec<NotifHandle<E>, 8>>,
}

impl<E> Channel<E>
where
    E: Clone,
{
    pub const fn new() -> Self {
        let registered = SafeCell::new(Vec::new());

        Channel {
            // executor,
            registered,
        }
    }

    pub fn register_task(&self, task: *const dyn Notifiable<E>) {
        self.registered.as_ref_mut().push(NotifHandle(task));
    }

    pub fn send_data(&self, data: E) {
        for t in self.registered.as_ref() {
            unsafe { (*t.0).notify(data.clone()) };
            // self.executor.enqueue_task(t.0);
        }
    }
}
