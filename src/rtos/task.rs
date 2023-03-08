use core::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TaskState {
    Ready,
    Running,
    Waiting
}

pub trait Task {
    fn get_priority(&self) -> u8;

    fn get_state(&self) -> TaskState;
    fn set_state(&self, state: TaskState);

    fn has_data(&self) -> bool;

    fn step(&self);
}

#[derive(Debug)]
pub struct TaskHandle {
    pub task: *const dyn Task,
    pub enqueue_time: u64,
}

unsafe impl Send for TaskHandle {}

impl Ord for TaskHandle {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_priority = unsafe { (*self.task).get_priority() };
        let other_priority = unsafe { (*other.task).get_priority() };

        if self_priority > other_priority {
            return Ordering::Greater;
        } else if self_priority < other_priority {
            return Ordering::Less;
        }

        let self_enqueue_time = self.enqueue_time;
        let other_enqueue_time = other.enqueue_time;

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
