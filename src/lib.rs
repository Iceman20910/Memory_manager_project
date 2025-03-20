/// Represents a memory block with start and end addresses and associated data.
#[derive(Debug, PartialEq)]
pub struct MemoryBlock {
    pub start: usize,
    pub end: usize,
    pub data: Vec<u8>,
}

impl MemoryBlock {
    pub fn new(start: usize, end: usize, data: Vec<u8>) -> Self {
        MemoryBlock { start, end, data }
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
}

impl std::fmt::Display for MemoryBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "0x{:04X} - 0x{:04X}: ALLOCATED (Size: {} bytes)\nData: {:?}",
            self.start,
            self.end,
            self.end - self.start,
            self.data
        )
    }
}

pub struct BuddyAllocator {
    free_blocks: Vec<(usize, usize)>, // (start, end)
}

impl BuddyAllocator {
    pub fn new() -> Self {
        let free_blocks = vec![(0, 65536)];
        BuddyAllocator { free_blocks }
    }

    pub fn allocate(&mut self, size: usize) -> Result<usize, String> {
        if size == 0 {
            return Err("Invalid size".to_string());
        }

        let aligned_size = size.next_power_of_two();

        for i in 0..self.free_blocks.len() {
            let (start, end) = self.free_blocks[i];
            if end - start >= aligned_size {
                self.free_blocks.remove(i);
                if end - start > aligned_size {
                    self.free_blocks.push((start + aligned_size, end));
                }
                return Ok(start);
            }
        }
        Err("Insufficient memory".to_string())
    }

    pub fn deallocate(&mut self, start: usize, size: usize) -> Result<(), String> {
        if size == 0 {
            return Err("Invalid size for deallocation".to_string());
        }
        let aligned_size = size.next_power_of_two();
        let end = start + aligned_size;

        if start >= end || end > 65536 {
            return Err("Invalid deallocation range".to_string());
        }

        self.free_blocks.push((start, end));
        self.merge_free_blocks();
        Ok(())
    }

    fn merge_free_blocks(&mut self) {
        self.free_blocks.sort_by_key(|&(start, _)| start);
        let mut i = 0;
        while i < self.free_blocks.len() - 1 {
            let (start1, end1) = self.free_blocks[i];
            let (start2, end2) = self.free_blocks[i + 1];
            if end1 == start2 {
                self.free_blocks[i] = (start1, end2);
                self.free_blocks.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    pub fn dump_free_blocks(&self) {
        for (start, end) in &self.free_blocks {
            println!("Free Block: 0x{:04X} - 0x{:04X}", start, end);
        }
    }
}

pub struct MemoryManager {
    allocator: BuddyAllocator,
    buffer: Vec<u8>,
    blocks: Vec<MemoryBlock>,
    next_id: usize,
}

impl MemoryManager {
    pub fn new() -> Self {
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
            self.buffer[new_start..new_start + data.len()].copy_from_slice(&data);
            self.allocator.deallocate(block.start, current_size).map_err(|e| e.to_string())?;
            block.start = new_start;
            block.end = new_start + data.len();
        } else {
            self.buffer[block.start..block.start + data.len()].copy_from_slice(&data);
        }
        block.set_data(data);
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
