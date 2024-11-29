mod handle_connection;

pub use handle_connection::handle_connection;
pub use colored::Colorize;

use handle_connection::thread;
use std::sync::{mpsc, Arc, Mutex};

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            println!("Worker {id} is alive");
            // let job = receiver.lock().unwrap().recv().unwrap();
            // // (receiver.lock().unwrap().recv().unwrap())();
            // // println!("Worker {id} completed the job");
            // job();

            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }

            
        });

        Worker { id, thread: Some(thread) }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;


pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero or less than zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
    
        let (tx, receiver) = mpsc::channel();

        let rx = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

        Self { workers, tx: Some(tx) }
    }
    
    // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {}

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        // println!("Job is send to the worker...");
        // println!("Pool lenght is {}", self.workers.len());
        
        self.tx.as_ref().unwrap().send(job).unwrap();
    }
    
}

impl Drop for ThreadPool {
    fn drop(&mut self) {

        // Dropping sender closes the channel, which indicates no more messages will be sent. 
        drop(self.tx.take());
        
        println!("{}","Shutting down has started.".red());
        
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
                println!("Worker {} is dead", worker.id);
            }
        }
        
        println!("{}","Shutting down is complete.".green());
    }
}