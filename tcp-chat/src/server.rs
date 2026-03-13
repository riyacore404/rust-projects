use std::net::TcpListener;
use std::io::Read;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080");

    let (mut stream, addr) = listener.accept().unwrap();
    println!("Client connected from {}", addr);

    loop {
        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer).unwrap();

        if n == 0 {
            println!("Client disconnected.");
            break;
        }

        let message = String::from_utf8_lossy(&buffer[..n]);
        if message.trim().eq_ignore_ascii_case("exit") {
            println!("Client requested to exit. Closing connection.");
            break; 
        }

        println!("Received message: {}", message.trim());
    }
}