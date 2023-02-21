use crate::rtos::task::TaskState;

pub trait TaskList: Ord {
    fn state(&self) -> &TaskState;
    fn dispatch(&mut self) -> TaskState;
}

