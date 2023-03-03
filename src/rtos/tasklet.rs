use core::default::Default;

use super::task::Task;
use super::taskdata::TaskData;

pub struct Tasklet<T>
where
    T: TaskData
{
    data: T,
    last_running_time: u32,
}

impl<T> Tasklet<T>
where
    T: TaskData + ~const Default
{
    pub const fn new(last_running_time: u32) -> Self {
        Tasklet { data: T::default(), last_running_time }
    }
}

impl<T: TaskData> Task for Tasklet<T> {
    fn get_last_running_time(&self) -> u32 {
        self.last_running_time
    }
}
