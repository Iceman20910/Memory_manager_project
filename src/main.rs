use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use memory_manager::MemoryManager;

fn main() {
    // Ensure stderr is flushed after each print
    std::io::stderr().flush().unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <command_file>", args[0]);
        std::io::stderr().flush().unwrap();
        return;
    }

    let file = File::open(&args[1]).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut memory_manager = MemoryManager::new();
    std::io::stderr().flush().unwrap();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        eprintln!("Processing command: {}", line);
        std::io::stderr().flush().unwrap();

        let mut parts = line.split_whitespace();

        match parts.next().unwrap() {
            "INSERT" => {
                let size = parts.next().unwrap().parse::<usize>().unwrap();
                let data = parts.next().unwrap().as_bytes().to_vec();
                match memory_manager.insert(size, data) {
                    Ok(id) => {
                        eprintln!("Allocated block with ID {}", id);
                        std::io::stderr().flush().unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        std::io::stderr().flush().unwrap();
                    }
                }
            }
            "DELETE" => {
                let id = parts.next().unwrap().parse::<usize>().unwrap();
                if let Err(e) = memory_manager.delete(id) {
                    eprintln!("Error: {}", e);
                    std::io::stderr().flush().unwrap();
                } else {
                    eprintln!("Deleted block with ID {}", id);
                    std::io::stderr().flush().unwrap();
                }
            }
            "READ" => {
                let id = parts.next().unwrap().parse::<usize>().unwrap();
                match memory_manager.find(id) {
                    Ok(block) => {
                        eprintln!("Block {} contains {:?}", id, block);
                        std::io::stderr().flush().unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        std::io::stderr().flush().unwrap();
                    }
                }
            }
            "UPDATE" => {
                let id = parts.next().unwrap().parse::<usize>().unwrap();
                let data = parts.next().unwrap().as_bytes().to_vec();
                if let Err(e) = memory_manager.update(id, data) {
                    eprintln!("Error: {}", e);
                    std::io::stderr().flush().unwrap();
                } else {
                    eprintln!("Updated block with ID {}", id);
                    std::io::stderr().flush().unwrap();
                }
            }
            "DUMP" => {
                memory_manager.dump();
                std::io::stderr().flush().unwrap();
            }
            _ => {
                eprintln!("Invalid command: {}", line);
                std::io::stderr().flush().unwrap();
            }
        }
    }
}
