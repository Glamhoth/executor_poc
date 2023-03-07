use core::cmp::Ordering;

#[derive(PartialEq, Copy, Clone)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
}

pub trait Task {
    fn get_state(&self) -> TaskState;
    fn set_state(&self, state: TaskState);

    fn get_last_running_time(&self) -> u64;
    fn set_last_running_time(&self, time: u64);

    fn step(&self);
}

#[derive(Debug)]
pub struct TaskHandle(pub *const dyn Task);

unsafe impl Send for TaskHandle {}
unsafe impl Sync for TaskHandle {}

impl Ord for TaskHandle {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_state = unsafe { (*self.0).get_state() };
        let other_state = unsafe { (*self.0).get_state() };

        match (self_state, other_state) {
            (TaskState::Ready, TaskState::Running) => return Ordering::Greater,
            (TaskState::Ready, TaskState::Blocked) => return Ordering::Greater,
            (TaskState::Running, TaskState::Ready) => return Ordering::Less,
            (TaskState::Blocked, TaskState::Ready) => return Ordering::Less,
            (_, _) => (),
        };

        let self_last_running_time = unsafe { (*self.0).get_last_running_time() };
        let other_last_running_time = unsafe { (*other.0).get_last_running_time() };

        if self_last_running_time > other_last_running_time {
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
