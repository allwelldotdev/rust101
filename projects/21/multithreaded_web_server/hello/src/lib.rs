//! Creating a Threadpool with Workers Implementation.

use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

#[allow(unused)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Creates a new ThreadPool.
    ///
    /// The `size` is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if `size == 0` or `size > 255`
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "size must be greater than 0");
        assert!(size < 129, "size must be less than or equal to 128");

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // Create some threads and store them in the vector
            let worker = Worker::new(id, Arc::clone(&receiver));
            workers.push(worker);
        }

        Self { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

#[allow(unused)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                // It truly is fantastic what happens here with the `Mutex<T>` type.
                // The call to `.lock()` blocks the thread until the lock is acquired;
                // when the former thread unlocks (or releases the lock).
                // Then, the thread that has acquired the lock will block again on the
                // call to `.recv()` until the thread receives the closure from the sender
                // via the `Threadpool::execute()` method.
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {id} got a job; executing.");
                job();
            }
        });
        Self { id, thread }
    }
}
