use std::net::TcpListener;
use std::io::Read;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080");

    let (mut stream, addr) = listener.accept().unwrap();
    println!("Client connected from {}", addr);

    let mut buffer = [0; 512];
    let n = stream.read(&mut buffer).unwrap();
    println!("Received message: {}", String::from_utf8_lossy(&buffer[..n]));
}