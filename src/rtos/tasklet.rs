use crate::rtos::critcell::CritCell;
use crate::rtos::safecell::SafeCell;
use crate::rtos::task::{Task, TaskState};
use crate::rtos::taskdata::TaskData;

pub struct Tasklet<T>
where
    T: TaskData + 'static,
{
    data: &'static SafeCell<T>,
    state: CritCell<TaskState>,
    last_running_time: CritCell<u64>,
    step_fn: &'static dyn Fn(&SafeCell<T>),
}

impl<T> Tasklet<T>
where
    T: TaskData,
{
    pub const fn new(data: &'static SafeCell<T>, step_fn: &'static dyn Fn(&SafeCell<T>)) -> Self {
        Tasklet {
            data,
            state: CritCell::new(TaskState::Ready),
            last_running_time: CritCell::new(0),
            step_fn,
        }
    }
}

impl<T: TaskData> Task for Tasklet<T> {
    fn get_state(&self) -> TaskState {
        self.state.lock(|s| *s)
    }

    fn set_state(&self, state: TaskState) {
        self.state.lock(|s| *s = state)
    }

    fn get_last_running_time(&self) -> u64 {
        self.last_running_time.lock(|t| *t)
    }

    fn set_last_running_time(&self, time: u64) {
        self.last_running_time.lock(|t| *t = time)
    }

    fn step(&self) {
        (self.step_fn)(self.data)
    }
}

unsafe impl<T> Sync for Tasklet<T> where T: Send + TaskData {}
