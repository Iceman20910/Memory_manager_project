use crate::free_block::FreeBlock;
use crate::allocated_block::AllocatedBlock;
use std::collections::HashMap;

pub struct MemoryManager {
    buffer: Vec<u8>,
    data_storage: HashMap<usize, Vec<u8>>, // Changed to HashMap
    free_blocks: Vec<FreeBlock>,
    allocated_blocks: Vec<AllocatedBlock>,
    next_id: usize,
    min_block_size: usize, // Minimum block size for allocation
}

impl MemoryManager {
    pub fn new() -> Self {
        let buffer_size = 65536;
        let buffer = vec![0u8; buffer_size];

        // Initially, the entire buffer is a free block
        let initial_free_block = FreeBlock { 
            start: 0, 
            end: buffer_size 
        };

        println!("Initializing MemoryManager with buffer size {}", buffer_size);

        MemoryManager {
            buffer,
            data_storage: HashMap::new(), // Initialize as an empty HashMap
            free_blocks: vec![initial_free_block],
            allocated_blocks: Vec::new(),
            next_id: 0,
            min_block_size: 64, // Set the minimum block size (example value)
        }
    }

    pub fn insert(&mut self, size: usize, data: Vec<u8>) -> Result<usize, String> {
        println!("Attempting to insert block of size {} with data {:?}", size, data);

        let start = self.allocate(size)?; // Allocate memory
        let end = start + size;
        let id = self.next_id;

        // Ensure the data is the correct size
        let mut padded_data = vec![0u8; size]; // Create a buffer of the allocated size
        let data_len = data.len();
        
        // Copy the original data into the padded buffer
        padded_data[..data_len].copy_from_slice(&data); // Copy original data
        self.data_storage.insert(id, padded_data.clone()); // Store the padded data with the ID as the key

        // Create allocated block
        let allocated_block = AllocatedBlock { id, start, end };
        self.allocated_blocks.push(allocated_block);

        self.next_id += 1;
        Ok(id)
    }

    pub fn allocate(&mut self, size: usize) -> Result<usize, String> {
        // Calculate the required block size
        let mut block_size = self.min_block_size;

        while block_size < size {
            block_size *= 2;
        }

        // Find the first free block that is large enough
        let index = self.free_blocks.iter()
            .position(|block| (block.end - block.start) >= block_size)
            .ok_or("No suitable block found".to_string())?;

        // Get the block to allocate
        let mut block = self.free_blocks.remove(index);

        // Split the block until it meets the size requirement
        while (block.end - block.start) > block_size {
            // Split the block into two halves
            let half_size = (block.end - block.start) / 2;
            let new_block = FreeBlock {
                start: block.start + half_size,
                end: block.end,
            };

            // Insert the new half back into the free list
            self.free_blocks.push(new_block);

            // Reduce the size of the current block
            block.end = block.start + half_size; // Update the end to reflect the new size
        }

        // Mark the block as allocated and return its start address
        Ok(block.start)
    }

    pub fn delete(&mut self, id: usize) -> Result<(), String> {
        println!("Attempting to delete block ID {}", id);
        
        // Find the block by ID
        let block_index = self.allocated_blocks.iter()
            .position(|block| block.id == id)
            .ok_or("Block not found".to_string())?;
        
        let block = self.allocated_blocks.remove(block_index);
        
        // Add back to free blocks
        self.free_blocks.push(FreeBlock { 
            start: block.start, 
            end: block.end 
        });

        // Remove corresponding data from the HashMap
        self.data_storage.remove(&id);

        Ok(())
    }

    pub fn update(&mut self, id: usize, data: Vec<u8>) -> Result<(), String> {
        println!("Attempting to update block ID {} with data {:?}", id, data);
        
        // Find the block by ID
        let block_index = self.allocated_blocks.iter()
            .position(|block| block.id == id)
            .ok_or("Block not found".to_string())?;
        
        // Clone the block (removed mutability)
        let block = self.allocated_blocks[block_index].clone();

        // Check if new data fits in existing block
        if data.len() > (block.end - block.start) {
            // Need to reallocate
            let new_start = self.allocate(data.len()).map_err(|e| e.to_string())?;
            let new_end = new_start + data.len();

            // Update buffer
            self.buffer[new_start..new_end].copy_from_slice(&data);

            // Free the old block
            self.delete(block.id)?;

            // Create new allocated block
            let new_block = AllocatedBlock {
                id: block.id,
                start: new_start,
                end: new_end,
            };
            self.allocated_blocks.push(new_block);
        } else {
            // Update buffer in-place
            self.buffer[block.start..block.start + data.len()].copy_from_slice(&data);
        }

        // Update data storage in the HashMap
        self.data_storage.insert(block.id, data);

        Ok(())
    }

    pub fn find(&self, id: usize) -> Result<&AllocatedBlock, String> {
        println!("Attempting to find block ID {}", id);
        self.allocated_blocks.iter()
            .find(|block| block.id == id)
            .ok_or("Block not found".to_string())
    }

    pub fn dump(&self) {
        println!("Memory Manager Dump:");
        
        // Print Free Blocks
        println!("Free Blocks:");
        for (i, free_block) in self.free_blocks.iter().enumerate() {
            println!(
                "  Free Block {}: Start: 0x{:04X}, End: 0x{:04X}, Size: {}",
                i, 
                free_block.start, 
                free_block.end,
                free_block.end - free_block.start // Calculate size here
            );
        }
        
        // Print Allocated Blocks
        println!("Allocated Blocks:");
        for block in &self.allocated_blocks {
            if let Some(data) = self.data_storage.get(&block.id) { // Get data from storage using the ID
                println!(
                    "  Block ID: {}, Start: 0x{:04X}, End: 0x{:04X}, Size: {}, Data: {:?}",
                    block.id,
                    block.start,
                    block.end,
                    block.end - block.start,
                    data
                );
            } else {
                println!(
                    "  Block ID: {}, Start: 0x{:04X}, End: 0x{:04X}, Size: {}, Data: <Missing>",
                    block.id,
                    block.start,
                    block.end,
                    block.end - block.start
                );
            }
        }
    }
}