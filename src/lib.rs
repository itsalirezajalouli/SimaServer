use std::{sync::mpsc, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender::<Job>,
}

struct Job;

impl ThreadPool {
    /// Creates a new ThreadPool
    /// 
    /// The size is the number of threads in the pool
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is `zero`.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
    
        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // Create & store threads
            workers.push(Worker::new(id, &receiver));
        }

        ThreadPool { workers, sender }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        // It's like saying "F needs to be able to:
        // - Run once (FnOnce())
        // - Be shared safely with others (Send)
        // - Last as long as needed ('static)"
    {
    }
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: &mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker { id, thread } 
    }
}
