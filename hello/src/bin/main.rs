extern crate hello;

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc, mpsc};
use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Received request");
        pool.execute(|| {
            handle_connection(stream);
        });
        break;
    }

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).unwrap();
    println!("Read {} bytes", bytes_read);
    let text = String::from_utf8_lossy(&buffer[..]);
    println!("Read {}", text);
    
    match read_uri(text.as_ref()) {
        Some("/") => serve_page(stream, 200, "hello.html"),
        Some("/sleep") => {
            thread::sleep(Duration::from_secs(5));
            serve_page(stream, 200, "hello.html");
        },
        _ => serve_page(stream, 404, "404.html"),
    }
}

fn read_uri(request: &str) -> Option<&str> {
    let first_line = request.split("\r\n").next();
    match first_line {
        Some(l) => l.split(" ").nth(1),
        None => None,
    }
}

fn serve_page(mut stream: TcpStream, code: u16, page: &str) {
    let mut file = File::open(page).unwrap();
    let mut file_text = String::new();
    file.read_to_string(&mut file_text).unwrap();
    let response = format!("HTTP/1.1 {} OK\r\n\r\n{}", code, file_text);

    let bytes_written = stream.write(response.as_bytes()).unwrap();
    println!("Wrote {} bytes", bytes_written);
    stream.flush().unwrap();
}
