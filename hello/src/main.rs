use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).unwrap();
    println!("Read {} bytes", bytes_read);
    let text = String::from_utf8_lossy(&buffer[..]);
    println!("Read text {}", text);
    let bytes_written = stream.write(String::from("HTTP/1.1 200 OK\r\n\r\n").as_bytes()).unwrap();
    println!("Wrote {} bytes", bytes_written);
    stream.flush().unwrap();
}
