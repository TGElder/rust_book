use std::thread;
use std::sync::{Mutex, Arc, mpsc};

type Job = Box<FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
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
            self.sender.send(Message::NewJob(Box::new(f))).unwrap();
    }
}

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        Worker { id, handle: Some(thread::spawn(move || {
            loop {
                match rx.lock().unwrap().recv().unwrap() {
                    Message::NewJob(job) => {
                        println!("Dispatched to worker {}", id);
                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} told to shut down", id);
                        break;
                    },
                }
            }
        })) }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            }
        }
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
