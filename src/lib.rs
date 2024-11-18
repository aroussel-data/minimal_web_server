use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        // create a threads array
        // fill it with threads by looping over size times and pushing to array

        let mut my_threads = Vec::with_capacity(size);

        for _ in 0..size {
            let mut new_thread = thread::spawn(|| {
                println!("created new thread!");
            });
            my_threads.push(new_thread);
        }
        return ThreadPool {
            threads: my_threads,
        };
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        println!("executing!");
    }
}
