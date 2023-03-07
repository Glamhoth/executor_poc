use core::cmp::Ordering;
use core::ffi::c_void;

pub trait Task {
    fn step(&self);
}

#[derive(Debug)]
pub struct TaskHandle(pub *const dyn Task);

unsafe impl Send for TaskHandle {}
unsafe impl Sync for TaskHandle {}

impl Ord for TaskHandle {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!();
    }
}

impl PartialOrd for TaskHandle {
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        todo!();
    }
}

impl Eq for TaskHandle {}

impl PartialEq for TaskHandle {
    fn eq(&self, _: &Self) -> bool {
        todo!();
    }
}
