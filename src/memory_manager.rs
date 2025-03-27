use crate::buddy_allocator::BuddyAllocator;
use crate::memory_block::MemoryBlock;

pub struct MemoryManager {
    allocator: BuddyAllocator,
    buffer: Vec<u8>,
    free_blocks: Vec<Option<MemoryBlock>>, // BS: create a free_block struct
    allocated_blocks: Vec<Option<MemoryBlock>>, // BS: create a allocated_block struct
    next_id: usize,
}

impl MemoryManager {
    pub fn new() -> Self {
        let buffer_size = 65536;
        let buffer = vec![0u8; buffer_size];

        println!(
            "Initializing MemoryManager with buffer size {}",
            buffer_size
        );
        MemoryManager {
            allocator: BuddyAllocator::new(),
            buffer,
            blocks: Vec::new(),
            next_id: 0,
        }
    }

    pub fn insert(&mut self, size: usize, data: Vec<u8>) -> Result<usize, String> {
        println!(
            "Attempting to insert block of size {} with data {:?}",
            size, data
        );
        let start = self.allocator.allocate(size).map_err(|e| e.to_string())?;
        let id = self.next_id;

        // Copy data to buffer
        self.buffer[start..start + data.len()].copy_from_slice(&data);

        println!(
            "Inserted block ID {} at start {} with size {}",
            id, start, size
        );

        // Find first empty slot or push new slot
        let block = MemoryBlock::new(start, start + size, data.clone());
        if let Some(slot) = self.blocks.iter_mut().find(|b| b.is_none()) {
            *slot = Some(block);
        } else {
            self.blocks.push(Some(block));
        }

        self.next_id += 1;
        Ok(id)
    }

    pub fn delete(&mut self, id: usize) -> Result<(), String> {
        println!("Attempting to delete block ID {}", id);

        // Find the block, ensuring it exists and is not already deleted
        let block = self
            .blocks
            .get_mut(id)
            .and_then(|b| b.take())
            .ok_or("Invalid ID or already deleted".to_string())?;

        println!(
            "Deallocating block at start {} with size {}",
            block.start,
            block.end - block.start
        );
        self.allocator
            .deallocate(block.start, block.end - block.start)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn update(&mut self, id: usize, data: Vec<u8>) -> Result<(), String> {
        println!("Attempting to update block ID {} with data {:?}", id, data);

        // Find the block, ensuring it exists and is not deleted
        let block = self
            .blocks
            .get_mut(id)
            .and_then(|b| b.as_mut())
            .ok_or("Invalid ID or block deleted".to_string())?;

        let current_size = block.end - block.start;

        if data.len() > current_size {
            println!("New data exceeds current block size. Reallocating.");
            let new_start = self
                .allocator
                .allocate(data.len())
                .map_err(|e| e.to_string())?;

            // Copy new data to buffer
            self.buffer[new_start..new_start + data.len()].copy_from_slice(&data);

            // Deallocate old block
            self.allocator
                .deallocate(block.start, current_size)
                .map_err(|e| e.to_string())?;

            block.start = new_start;
            block.end = new_start + data.len();
        } else {
            // Copy data to existing block's buffer space
            self.buffer[block.start..block.start + data.len()].copy_from_slice(&data);
        }

        block.set_data(data);
        println!("Updated block ID {} successfully", id);
        Ok(())
    }

    pub fn find(&self, id: usize) -> Result<&MemoryBlock, String> {
        println!("Attempting to find block ID {}", id);
        self.blocks
            .get(id) // BS: store id inside of allocated_block and search for matching id
            .and_then(|b| b.as_ref())
            .ok_or("Block not found".to_string())
    }

    fn get_data(&self, block: MemoryBlock) -> String {}

    pub fn dump(&self) {
        println!("Memory Manager Dump:");
        let active_blocks = self.blocks.iter().filter(|b| b.is_some()).count();
        println!("Total Blocks: {}", active_blocks);

        for (id, block_option) in self.blocks.iter().enumerate() {
            if let Some(block) = block_option {
                println!(
                    "Block ID: {}, Start: {}, End: {}, Size: {}, Data: {:?}",
                    id,
                    block.start,
                    block.end,
                    block.end - block.start,
                    block.get_data() // BS: call self.get_data(block);
                );
            }
        }
    }
}
