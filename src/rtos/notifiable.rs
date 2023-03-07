use crate::rtos::task::Task;

pub trait Notifiable<E> : Task {
    fn notify(&self, data: E);
}

pub struct NotifHandle<E>(pub *const dyn Notifiable<E>);

unsafe impl<E> Sync for NotifHandle<E> where E: Send {}
unsafe impl<E> Send for NotifHandle<E> {}
