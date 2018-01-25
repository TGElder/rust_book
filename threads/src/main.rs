use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals: Vec<String> = vec!["hi", "from", "the", "thread"].iter()
            .map(|s| String::from(*s))
            .collect();
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals: Vec<String> = vec!["more", "messages", "for", "you"].iter()
            .map(|s| String::from(*s))
            .collect();
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
