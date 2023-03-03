use core::any::Any;
use core::cmp::Ordering;

use crate::rtos::task::Task;
use crate::rtos::taskdata::TaskData;

pub struct Tasklet<T>
where
    T: TaskData,
{
    task_data: T,
    last_running_time: u64,
    step_function: &'static dyn Fn(),
}

impl<T> Tasklet<T>
where
    T: TaskData,
{
    pub const fn new(task_data: T, step_function: &'static dyn Fn()) -> Self {
        Tasklet {
            task_data,
            step_function,
            last_running_time: 0,
        }
    }
}

impl<T> Task for Tasklet<T>
where
    T: TaskData,
{
    fn get_last_running_time(&self) -> u64 {
        self.last_running_time
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<T> Ord for Tasklet<T>
where
    T: TaskData,
{
    fn cmp(&self, _: &Self) -> Ordering {
        todo!();
    }
}

impl<T> PartialOrd for Tasklet<T>
where
    T: TaskData,
{
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        todo!();
    }
}

impl<T> Eq for Tasklet<T> where T: TaskData {}

impl<T> PartialEq for Tasklet<T>
where
    T: TaskData,
{
    fn eq(&self, _: &Self) -> bool {
        todo!();
    }
}
