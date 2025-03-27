use crate::buddy_allocator::BuddyAllocator;
use crate::free_block::FreeBlock;
use crate::allocated_block::AllocatedBlock;
use std::collections::HashMap;

pub struct MemoryManager {
    allocator: BuddyAllocator,
    buffer: Vec<u8>,
    data_storage: HashMap<usize, Vec<u8>>, // Changed to HashMap
    free_blocks: Vec<FreeBlock>,
    allocated_blocks: Vec<AllocatedBlock>,
    next_id: usize,
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
            allocator: BuddyAllocator::new(),
            buffer,
            data_storage: HashMap::new(), // Initialize as an empty HashMap
            free_blocks: vec![initial_free_block],
            allocated_blocks: Vec::new(),
            next_id: 0,
        }
    }

    pub fn insert(&mut self, size: usize, data: Vec<u8>) -> Result<usize, String> {
        println!("Attempting to insert block of size {} with data {:?}", size, data);

        // Allocate memory
        let start = self.allocator.allocate(size).map_err(|e| e.to_string())?;
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

        // Update free blocks
        self.update_free_blocks(start, end);

        // Copy data to buffer
        self.buffer[start..end].copy_from_slice(&padded_data); // Ensure size matches

        self.next_id += 1;
        Ok(id)
    }

    fn update_free_blocks(&mut self, start: usize, end: usize) {
        let mut updated_free_blocks = Vec::new();

        for free_block in &self.free_blocks {
            // If the new block is completely outside this free block, keep it
            if end <= free_block.start || start >= free_block.end {
                updated_free_blocks.push(free_block.clone());
            } else {
                // Handle partial overlaps
                // Add free block before allocation if exists
                if start > free_block.start {
                    updated_free_blocks.push(FreeBlock {
                        start: free_block.start,
                        end: start,
                    });
                }

                // Add free block after allocation if exists
                if end < free_block.end {
                    updated_free_blocks.push(FreeBlock {
                        start: end,
                        end: free_block.end,
                    });
                }
            }
        }

        // Replace old free blocks with updated ones
        self.free_blocks = updated_free_blocks;
    }

    pub fn delete(&mut self, id: usize) -> Result<(), String> {
        println!("Attempting to delete block ID {}", id);
        
        // Find the block by ID
        let block_index = self.allocated_blocks.iter()
            .position(|block| block.id == id)
            .ok_or("Block not found".to_string())?;
        
        let block = self.allocated_blocks.remove(block_index);
        
        // Deallocate memory
        self.allocator.deallocate(block.start, block.end - block.start)
            .map_err(|e| e.to_string())?;
        
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
        
        // Clone the block to avoid mutable borrow conflict
        let mut block = self.allocated_blocks[block_index].clone();

        // Check if new data fits in existing block
        if data.len() > block.end - block.start {
            // Need to reallocate
            let new_start = self.allocator.allocate(data.len()).map_err(|e| e.to_string())?;
            let new_end = new_start + data.len();

            // Update buffer
            self.buffer[new_start..new_end].copy_from_slice(&data);

            // Deallocate old block
            self.allocator.deallocate(block.start, block.end - block.start)
                .map_err(|e| e.to_string())?;

            // Modify the cloned block
            block.start = new_start;
            block.end = new_end;

            // Update free blocks
            self.update_free_blocks(new_start, new_end);
        } else {
            // Update buffer in-place
            self.buffer[block.start..block.start + data.len()].copy_from_slice(&data);
        }

        // Update the original allocated block in place
        self.allocated_blocks[block_index] = block.clone(); // Clone again for assignment

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
                free_block.end - free_block.start
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