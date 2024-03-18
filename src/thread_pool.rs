use std::io;
use std::io::ErrorKind;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};
pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() -> io::Result<()> + Send + 'static>;

impl ThreadPool {
    /// Create a new thread pool with `size` threads.
    /// # Panics if 0 threads
    pub fn build(size: usize) -> io::Result<Self> {
        if size == 0 { return Err(io::Error::from(io::ErrorKind::Other)) };

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut threads = Vec::with_capacity(size);
        for i in 0..size {
            threads.push(Worker::new(i, Arc::clone(&receiver)));
        }
        
        Ok(ThreadPool { threads, sender })
    }

    pub fn execute<F>(&self, f: F) -> io::Result<()>
    where
        F: FnOnce() -> io::Result<()>,
        F: Send + 'static
    {
        let job = Box::new(f);

        // TODO: Allow errors from jobs `T` to flow from executing threads
        // back to main()
        //TODO: Allow sender.send() error to flow back to main and be interpreted
        // in overarching error type.
        self.sender.send(job).map_err(|_| {io::Error::from(ErrorKind::Other)})?;
        Ok(())
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            //TODO: Report job failures, perhaps through a channel.
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            let _ = job();
        });

        Worker {
            id: id,
            thread,
        }
    }
}
