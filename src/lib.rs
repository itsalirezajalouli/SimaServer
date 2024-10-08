use std::{
    sync::{mpsc, Arc,Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender::<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

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

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        // it's more memory efficient cause Vec::new resizes as elements are inserted!

        for id in 0..size {
            // Create & store threads
            workers.push(Worker::new(id, Arc::clone(&receiver)));
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
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct Worker {
    // Worker struct sends code from threadpool to thread
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
    // Join handle detaches associated thread when it's dropped
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().expect("A worker got poisened!").recv().unwrap();
            
            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker {
            id,
            thread: Some(thread)} 
    }
}
