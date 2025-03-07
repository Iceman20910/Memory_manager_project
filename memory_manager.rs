use std::collections::HashMap;
use crate::buddy_allocator::BuddyAllocator; 
use crate::memory_block::MemoryBlock;

pub struct MemoryManager {
    allocator: BuddyAllocator,
    blocks: HashMap<usize, MemoryBlock>,
    next_id: usize,
}

impl MemoryManager {
    pub fn new() -> Self {
        MemoryManager {
            allocator: BuddyAllocator::new(),
            blocks: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn insert(&mut self, size: usize, data: Vec<u8>) -> Result<usize, String> {
        let start = self.allocator.allocate(size).map_err(|e| e.to_string())?;
        let id = self.next_id;
        self.blocks.insert(id, MemoryBlock::new(start, start + size, data));
        self.next_id += 1;
        Ok(id)
    }

    pub fn delete(&mut self, id: usize) -> Result<(), String> {
        let block = self.blocks.remove(&id).ok_or("Invalid ID")?;
        self.allocator.deallocate(block.start).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn update(&mut self, id: usize, data: Vec<u8>) -> Result<(), String> {
        let block = self.blocks.get_mut(&id).ok_or("Invalid ID")?;
        let current_size = block.end - block.start;

        if data.len() > current_size {
            let new_start = self.allocator.allocate(data.len()).map_err(|e| e.to_string())?;
            block.data = data.clone();
            self.allocator.deallocate(block.start).map_err(|e| e.to_string())?;
            block.start = new_start;
            block.end = new_start + data.len();
        } else {
            block.data = data.clone();
        }
        Ok(())
    }

    pub fn find(&self, id: usize) -> Result<&MemoryBlock, String> {
        self.blocks.get(&id).ok_or("Block not found".to_string())
    }

    pub fn dump(&self) {
        for (id, block) in &self.blocks {
            println!("Block ID: {}, Start: {}, End: {}, Size: {}", id, block.start, block.end, block.end - block.start);
        }
    }
}