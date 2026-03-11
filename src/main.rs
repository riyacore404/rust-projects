use std::io;
use std::io::Write; // for flushing stdout

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new(); // user input
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim(); // Remove trailing newline

        // Split the input into parts (command and arguments)
        let parts: Vec<&str> = input.split_whitespace().collect();

        // If nothing was typed, loop again
        if parts.is_empty() { continue; }

        match parts[0] {
            "exit" => { println!("Bye!"); break; }
            "set" => println!("set command received"),
            "get" => println!("get command received"),
            "delete" => println!("delete command received"),
            "list" => println!("list command received"),

            _ => println!("Unknown command: {}", parts[0]),
        }
    }
}
