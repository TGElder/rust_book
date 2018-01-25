use std::sync::{Mutex, Arc};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
   let m = Arc::new(Mutex::new(0));
   let mut handles = vec![];
   for _ in 0..10 {
       let m2 = Arc::clone(&m);
       let handle = thread::spawn(move || {
           let mut num = m2.lock().unwrap();
           *num += 1
       });
       handles.push(handle);
   }

   for handle in handles {
       handle.join().unwrap();
   }

   println!("m = {:?}", m);

   // Deadlock test
   let r1 = Arc::new(Mutex::new(0));
   let r2 = Arc::new(Mutex::new(0));

   let r1b = Arc::clone(&r1);
   let r2b = Arc::clone(&r2);

   let handle1 = thread::spawn(move || {
       let mut val = r1.lock().unwrap();
       thread::sleep(Duration::from_secs(1));
       println!("{:?}", r2b);
       r2b.lock().unwrap();
       *val += 1;
   });

   let handle2 = thread::spawn(move || {
       let mut val = r2.lock().unwrap();
       thread::sleep(Duration::from_secs(1));
       println!("{:?}", r1b);
       r1b.lock().unwrap();
       *val += 1;
   });

   handle1.join().unwrap();
   handle2.join().unwrap();
   
}
