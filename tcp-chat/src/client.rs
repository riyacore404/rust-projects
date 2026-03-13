use std::net::TcpStream;
use std::io;
use std::io::Write;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    println!("Connected to server at 127.0.0.1:8080");

    loop {
        print!("> ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout"); // Ensure the prompt is printed before waiting for input

        let mut input = String::new(); // user input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        if input.trim().is_empty() {
            continue; // Skip empty input
        }

        if input.trim().eq_ignore_ascii_case("exit") {
            stream.write_all(b"exit").unwrap();
            println!("Exiting...");
            break; // Exit the loop if the user types "exit"
        }

        stream.write_all(input.as_bytes()).unwrap();
    }
}