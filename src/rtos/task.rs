pub trait Task {
    fn get_last_running_time(&self) -> u32;

    fn as_task(&'static self) -> *const dyn Task where Self: Sized {
        self
    }
}
