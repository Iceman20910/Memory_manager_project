use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

// use memory_manager::MemoryManager; // fix this

fn main() {
    println!("Starting memory manager application"); // Debug print

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <command_file>", args[0]);
        return;
    }

    let file = File::open(&args[1]).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut memory_manager = MemoryManager::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        println!("Processing command: {}", line); // Debug print

        let mut parts = line.split_whitespace();

        match parts.next().unwrap() {
            "INSERT" => {
                let size = parts.next().unwrap().parse::<usize>().unwrap();
                let data = parts.next().unwrap().as_bytes().to_vec();
                match memory_manager.insert(size, data) {
                    Ok(id) => println!("Allocated block with ID {}", id),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "DELETE" => {
                let id = parts.next().unwrap().parse::<usize>().unwrap();
                if let Err(e) = memory_manager.delete(id) {
                    println!("Error: {}", e);
                }
            }
            "READ" => {
                let id = parts.next().unwrap().parse::<usize>().unwrap();
                match memory_manager.find(id) {
                    Ok(block) => println!("Block {} contains {:?}", id, block),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "UPDATE" => {
                let id = parts.next().unwrap().parse::<usize>().unwrap();
                let data = parts.next().unwrap().as_bytes().to_vec();
                if let Err(e) = memory_manager.update(id, data) {
                    println!("Error: {}", e);
                }
            }
            "DUMP" => {
                memory_manager.dump();
            }
            _ => {
                println!("Invalid command");
            }
        }
    }

    println!("Memory manager application finished"); // Debug print
}
