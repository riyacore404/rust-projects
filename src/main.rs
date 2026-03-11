use std::io;
use std::io::Write; // for flushing stdout
use std::collections::HashMap; // for in-memory key-value store

fn main() {
    let mut storage: HashMap<String, String> = HashMap::new(); // In-memory key-value store

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
            "set" => {
                if parts.len() < 3 {
                    println!("Usage: set <key> <value>")
                } else {
                    storage.insert(parts[1].to_string(), parts[2].to_string());
                    println!("Set key '{}' to value '{}'", parts[1], parts[2]);
                }
            },
            "get" => {
                if parts.len() < 2 {
                    println!("Usage: get <key>")
                } else {
                    let key = parts[1];
                    match storage.get(key) {
                        Some(value) => println!("Value for key '{}': '{}'", key, value),
                        None => println!("Key '{}' not found", key),
                    }
                }
            },
            "delete" => {
                if parts.len() < 2 {
                    println!("Usage: delete <key>")
                } else {
                    let key = parts[1];
                    if storage.remove(key).is_some() {
                        println!("Deleted key '{}'", key);
                    } else {
                        println!("Key '{}' not found", key);
                    }
                }
            },
            "list" => {
                if storage.is_empty() {
                    println!("No keys in storage");
                } else {
                    println!("Keys in storage:");
                    for (key, value) in storage.iter() {
                        println!("- {}: {}", key, value);
                    }
                }
            },

            _ => println!("Unknown command: {}", parts[0]),
        }
    }
}
