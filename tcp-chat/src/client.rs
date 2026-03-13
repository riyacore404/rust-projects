use std::net::TcpStream;
use std::io::Write;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    println!("Connected to server at 127.0.0.1:8080");

    stream.write_all(b"Hello, server!").unwrap();
}