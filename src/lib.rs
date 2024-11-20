use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
    sender: Option<Sender<Job>>,
    workers: Vec<Worker>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (tx, rx) = mpsc::channel();

        // threadpool holds on to sender aka tx
        // each worker will hold on to receiver aka rx

        let mut workers_vector = Vec::with_capacity(size);

        let rx = Arc::new(Mutex::new(rx));

        for id in 0..size {
            let my_worker = Worker::new(id, Arc::clone(&rx));
            workers_vector.push(my_worker);
        }
        return ThreadPool {
            sender: Some(tx.clone()),
            workers: workers_vector,
        };
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.handle.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let task = receiver.lock().unwrap().recv();

            match task {
                Ok(job) => {
                    println!("Worker {id} received job...executing");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected...shutting down");
                    break;
                }
            }
        });

        return Worker {
            id,
            handle: Some(thread),
        };
    }
}
