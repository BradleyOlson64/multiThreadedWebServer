use std::io;
use std::io::ErrorKind;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};
pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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
        
        Ok(ThreadPool { threads, sender: Some(sender) })
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
        self.sender.as_ref().unwrap().send(job).map_err(|_| {io::Error::from(ErrorKind::Other)})?;
        Ok(())
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.threads {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            //TODO: Report job failures, perhaps through a channel.
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                let _ = job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}
