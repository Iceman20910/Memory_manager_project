pub struct MemoryManager {
    allocator: BuddyAllocator,
    buffer: Vec<u8>,
    blocks: Vec<MemoryBlock>,
    next_id: usize,
}

impl MemoryManager {
    pub fn new() -> Self {
        // Initialize buffer with zeros and a fixed size
        let buffer_size = 65536;
        let buffer = vec![0u8; buffer_size];

        MemoryManager {
            allocator: BuddyAllocator::new(),
            buffer,
            blocks: Vec::new(),
            next_id: 0,
        }
    }

    pub fn insert(&mut self, size: usize, data: Vec<u8>) -> Result<usize, String> {
        let start = self.allocator.allocate(size).map_err(|e| e.to_string())?;
        let id = self.next_id;
        self.blocks.push(MemoryBlock::new(start, start + size, data));
        self.next_id += 1;
        Ok(id)
    }

    pub fn delete(&mut self, id: usize) -> Result<(), String> {
        if id >= self.next_id {
            return Err("Invalid ID".to_string());
        }
        let block = self.blocks.remove(id);
        self.allocator.deallocate(block.start, block.end - block.start)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn update(&mut self, id: usize, data: Vec<u8>) -> Result<(), String> {
        let block = self.blocks.get_mut(id).ok_or("Invalid ID")?;
        let current_size = block.end - block.start;

        if data.len() > current_size {
            let new_start = self.allocator.allocate(data.len()).map_err(|e| e.to_string())?;
            // Copy data into the buffer
            self.buffer[new_start..new_start + data.len()].copy_from_slice(&data);
            self.allocator.deallocate(block.start, current_size).map_err(|e| e.to_string())?;
            block.start = new_start;
            block.end = new_start + data.len();
        } else {
            // Update data in the buffer
            self.buffer[block.start..block.start + data.len()].copy_from_slice(&data);
        }
        block.set_data(data); // Update the MemoryBlock's data
        Ok(())
    }

    pub fn find(&self, id: usize) -> Result<&MemoryBlock, String> {
        self.blocks.get(id).ok_or("Block not found".to_string())
    }

    pub fn dump(&self) {
        for (id, block) in self.blocks.iter().enumerate() {
            println!(
                "Block ID: {}, Start: {}, End: {}, Size: {}",
                id,
                block.start,
                block.end,
                block.end - block.start
            );
        }
    }
}