pub trait Queue<T> {
    fn enqueue(&self, elem: T) -> Result<(), T>;
    fn dequeue(&self) -> Option<T>;
}
