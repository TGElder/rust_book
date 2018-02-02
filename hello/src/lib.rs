use std::thread;
use std::sync::{Mutex, Arc, mpsc};

type Job = Box<FnBox + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}


impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let (sender, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&rx), Arc::new(Mutex::new(false))));
        }

        ThreadPool{ workers , sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F) 
        where F: FnOnce() + Send + 'static
        {
            if let Some(ref sender) = self.sender {
                sender.send(Box::new(f)).unwrap();
            }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Dropping sender");
        if let Some(sender) = self.sender.take() {
            drop(sender);
        }
        let workers = &mut self.workers;
        for mut worker in workers.iter_mut() {
            {
                println!("Telling worker to stop");
                let mut val = worker.stop.lock().unwrap();
                *val = true;
            }
            if let Some(handle) = worker.handle.take() {
                println!("Waiting for worker to join");
                handle.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
    stop: Arc<Mutex<bool>>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>, stop: Arc<Mutex<bool>>) -> Worker {
        let stop2 = Arc::clone(&stop);
        Worker { id, handle: Some(thread::spawn(move || {
            while *stop.lock().unwrap() == false  {
                let job = rx.lock().unwrap().recv().unwrap();
                println!("Dispatched to worker {}", id);
                job.call_box();
            }
            println!("Terminating worker {}", id);
        })), stop: stop2 }
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
