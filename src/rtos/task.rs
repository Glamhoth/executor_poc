use core::cmp::Ordering;

pub trait Task {
    fn get_priority(&self) -> u8;

    fn step(&self);
}

#[derive(Debug)]
pub struct TaskHandle(pub *const dyn Task, pub u64);

unsafe impl Send for TaskHandle {}

impl Ord for TaskHandle {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_priority = unsafe { (*self.0).get_priority() };
        let other_priority = unsafe { (*other.0).get_priority() };

        if self_priority > other_priority {
            return Ordering::Greater;
        } else if self_priority < other_priority {
            return Ordering::Less;
        }

        let self_enqueue_time = self.1;
        let other_enqueue_time = other.1;

        if self_enqueue_time > other_enqueue_time {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
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
