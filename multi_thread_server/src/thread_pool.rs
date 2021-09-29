#![allow(dead_code)]

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size); //slightly effective than Vec::new() due to allocate size memory

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        ThreadPool { workers, sender }
    }

    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn execute(&self, f: impl FnOnce() + Send + 'static) {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        } //ensure all worker get Terminate, send it as many as threads
        //if mixing these loop into one, dead lock occurred. consider
        //  1 - Working
        //  2 - Idle

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take()
            //take() pop the value and replace it None
            {
                thread.join().unwrap()
            }
        }
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

//thread represantation word in thread pool
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = Some(thread::spawn(move || {
            loop {
                //loop and if job receiving, exec - thread infinitely exec
                //lock() -> wait and block exec for the Mutex until aquired
                //recv() -> wait and block exec for Msg until got
                //unwarap() panics if the Mutex is poisoned or Sender dropped
                let job = receiver.lock().unwrap().recv().unwrap();

                match job {
                    Message::NewJob(job) => {
                        println!("the {}th worker get the job; executing", id);

                        job.call_box()
                    }
                    Message::Terminate => {
                        println!("the {}th worker was told to terminate", id);

                        break;
                    }
                }

                println!("the {}th worker completed execution", id);
            }
        }));
        Worker { id, thread }
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}
