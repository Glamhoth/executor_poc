use core::default::Default;

use super::task::Task;
use super::taskdata::TaskData;
use super::safecell::SafeCell;

pub struct Tasklet<T>
where
    T: TaskData + 'static,
{
    data: T,
    last_running_time: u32,
    step_fn: &'static dyn Fn(&SafeCell<T>),
}

impl<T> Tasklet<T>
where
    T: TaskData + ~const Default,
{
    pub const fn new(step_fn: &'static dyn Fn(&SafeCell<T>)) -> Self {
        Tasklet {
            data: T::default(),
            last_running_time: 0,
            step_fn,
        }
    }
}

impl<T: TaskData> Task for Tasklet<T> {
    fn get_last_running_time(&self) -> u32 {
        self.last_running_time
    }
}

unsafe impl<T> Sync for Tasklet<T> where T: Send + TaskData {}
