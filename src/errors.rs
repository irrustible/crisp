
pub enum RuntimeError<T> {
    /// We collected all the garbage and we still couldn't allocate
    /// the memory to store this value on the heap.
    OutOfMemory(T),
}

