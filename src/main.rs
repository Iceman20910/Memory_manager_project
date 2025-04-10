use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use memory_manager::MemoryManager;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <command_file>", args[0]);
        return;
    }

    let file = File::open(&args[1]).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut memory_manager = MemoryManager::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        eprintln!("Processing command: {}", line);

        let mut parts = line.split_whitespace();

        match parts.next().unwrap() {
            "INSERT" => {
                let size = parts.next().unwrap().parse::<usize>().unwrap();
                let data = parts.next().unwrap().as_bytes().to_vec();
                match memory_manager.insert(size, data) {
                    Ok(id) => eprintln!("Allocated block with ID {}", id),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "DELETE" => {
                let id = parts.next().unwrap().parse::<usize>().unwrap();
                if let Err(e) = memory_manager.delete(id) {
                    eprintln!("Error: {}", e);
                } else {
                    eprintln!("Deleted block with ID {}", id);
                }
            }
            "READ" => {
                let id = parts.next().unwrap().parse::<usize>().unwrap();
                match memory_manager.find(id) {
                    Ok(block) => eprintln!("Block {} contains {:?}", id, block),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "UPDATE" => {
                let id = parts.next().unwrap().parse::<usize>().unwrap();
                let data = parts.next().unwrap().as_bytes().to_vec();
                if let Err(e) = memory_manager.update(id, data) {
                    eprintln!("Error: {}", e);
                } else {
                    eprintln!("Updated block with ID {}", id);
                }
            }
            "DUMP" => {
                memory_manager.dump();
            }
            _ => {
                eprintln!("Invalid command: {}", line);
            }
        }
    }
}