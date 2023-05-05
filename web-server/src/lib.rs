use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        // FnOnce because the callback will be called once
        // Send because it will be moved across threads
        // 'static because we don't know how long it will be alive
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        // sender.as_ref because unwrap takes ownership of the sender, we don't want
        // to move it, just unwrap a reference
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // take the Some(sender) from the Option and replace it with a None
        // the drop the sender
        // dropping the sender closes the channel, which indicates that no more
        // messages will be sent. When that happens, the recv() in the
        // workers loops will return an error
        drop(self.sender.take());

        while !self.workers.is_empty() {
            if let Some(worker) = self.workers.pop() {
                println!("Shutting down worker {}", worker.id);
                worker.thread.join().unwrap();
            }
        }

        // The book mead the worker.thread an Option<T> and
        // implemented the shutdown logic as follows.
        // However, I didn't understand why they didn't just pop the workers
        // from the vector and join them
        // for worker in &mut self.workers {
        //     println!("Shutting down worker {}", worker.id);

        //     if let Some(thread) = worker.thread.take() {
        //         thread.join().unwrap();
        //     }
        // }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
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

        Worker { id, thread }
    }
}
