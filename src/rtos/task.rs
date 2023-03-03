use core::any::Any;

pub trait Task {
    fn get_last_running_time(&self) -> u64;

    fn as_any(&mut self) -> &mut dyn Any;
}
