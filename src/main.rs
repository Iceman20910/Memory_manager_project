use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use memory_manager::MemoryManager;

fn main() {
    let mut mm = MemoryManager::new(); // Initialize MemoryManager with default size
    mm.insert(3, b"hi".to_vec()).expect("Insert failed"); // Insert data
    println!("{}", mm.find(0).expect("Find failed").to_string()); // Use to_string for display
    println!();
    mm.dump(); // Print memory dump
    
    // Handle command file if provided
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let file = File::open(&args[1]).expect("Failed to open file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            eprintln!("Processing command: {}", line);

            let mut parts = line.split_whitespace();

            match parts.next().unwrap().to_uppercase().as_str() {
                "INSERT" => {
                    let size = parts.next().unwrap().parse::<usize>().unwrap();
                    let data = parts.next().unwrap().as_bytes().to_vec();
                    match mm.insert(size, data) {
                        Ok(id) => eprintln!("Allocated block with ID {}", id),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                "DELETE" => {
                    let id = parts.next().unwrap().parse::<usize>().unwrap();
                    if let Err(e) = mm.delete(id) {
                        eprintln!("Error: {}", e);
                    } else {
                        eprintln!("Deleted block with ID {}", id);
                    }
                }
                "UPDATE" => {
                    let id = parts.next().unwrap().parse::<usize>().unwrap();
                    let data = parts.next().unwrap().as_bytes().to_vec();
                    if let Err(e) = mm.update(id, data) {
                        eprintln!("Error: {}", e);
                    } else {
                        eprintln!("Updated block with ID {}", id);
                    }
                }
                "DUMP" => {
                    mm.dump();
                }
                _ => {
                    eprintln!("Invalid command: {}", line);
                }
            }
        }
    } else {
        eprintln!("Usage: {} <command_file>", args[0]);
    }
}