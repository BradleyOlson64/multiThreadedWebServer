use std::io;
pub struct ThreadPool;

impl ThreadPool {
    /// Create a new thread pool with `size` threads.
    /// # Panics if 0 threads
    pub fn build(size: usize) -> io::Result<Self> {
        if size == 0 { return Err(io::Error::from(io::ErrorKind::Other)) };
        Ok(ThreadPool)
    }

    pub fn execute<F, T>(&self, f: F)
    where
        F: FnOnce() -> T,
        F: Send + 'static
    {
        
    }
}
