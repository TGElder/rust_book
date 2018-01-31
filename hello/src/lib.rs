use std::thread;
use std::sync::{Mutex, Arc, mpsc};

type Job = Box<FnBox + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}


impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let (sender, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&rx)));
        }

        ThreadPool{ workers , sender }
    }

    pub fn execute<F>(&self, f: F) 
        where F: FnOnce() + Send + 'static
        {
            self.sender.send(Box::new(f)).unwrap();
    }
}

struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        Worker { id, handle: thread::spawn(move || {
            loop {
                let job = rx.lock().unwrap().recv().unwrap();
                println!("Dispatched to worker {}", id);
                job.call_box();
            }
        }) }
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}
