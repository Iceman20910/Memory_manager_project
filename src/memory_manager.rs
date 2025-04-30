use crate::allocated_block::AllocatedBlock;
use crate::free_block::FreeBlock;
use crate::buddy_allocator::BuddyAllocator;

/// Represents a memory block with start and end addresses
#[derive(Debug, PartialEq)]
pub enum MemoryBlock {
    Free(FreeBlock),
    Allocated(AllocatedBlock),
}

impl MemoryBlock {
    pub fn start(&self) -> usize {
        match self {
            MemoryBlock::Free(block) => block.start,
            MemoryBlock::Allocated(block) => block.start,
        }
    }

    pub fn end(&self) -> usize {
        match self {
            MemoryBlock::Free(block) => block.end,
            MemoryBlock::Allocated(block) => block.end,
        }
    }

    pub fn size(&self) -> usize {
        self.end() - self.start()
    }
}

pub struct MemoryManager {
    allocator: BuddyAllocator,
    buffer: Vec<u8>,
    data_storage: Vec<(Vec<u8>, usize)>, // Store actual data length
    blocks: Vec<MemoryBlock>,
    next_id: usize,
}

impl MemoryManager {
    pub fn new() -> Self {
        let buffer_size = 65536; // Total buffer size
        let buffer = vec![0u8; buffer_size]; // Initialize buffer with zeros
        let initial_free_block = MemoryBlock::Free(FreeBlock::new(0, buffer_size));
        println!("Initializing MemoryManager with buffer size {}", buffer_size);

        MemoryManager {
            allocator: BuddyAllocator::new(),
            buffer,
            data_storage: Vec::new(),
            blocks: vec![initial_free_block],
            next_id: 0,
        }
    }

    // Method to return a reference to the buffer
    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn insert(&mut self, size: usize, data: Vec<u8>) -> Result<usize, String> {
        let rounded_size = Self::round_up_to_power_of_two(size);
        let start = self.allocator.allocate(rounded_size).map_err(|e| e.to_string())?;
        let end = start + rounded_size;

        let id = self.next_id;
        let data_index = self.data_storage.len();

        // Prepare padded data and store actual data length
        let mut padded_data = vec![0u8; rounded_size];
        let data_len = data.len();
        padded_data[..data_len].copy_from_slice(&data);
        self.data_storage.push((padded_data, data_len)); // Store padded data and actual length

        let allocated_block = MemoryBlock::Allocated(AllocatedBlock::new(id, start, end, data_index));

        let block_index = self.blocks.iter()
            .position(|block| match block {
                MemoryBlock::Free(free_block) => free_block.start <= start && free_block.end >= end,
                _ => false,
            })
            .ok_or("No suitable free block found".to_string())?;

        let original_block = self.blocks.remove(block_index);
        self.blocks.push(allocated_block);

        match original_block {
            MemoryBlock::Free(free_block) => {
                let remaining_size = free_block.size() - rounded_size;
                if remaining_size > 0 {
                    let (_allocated_part, remaining_part) = free_block.split(rounded_size);
                    self.blocks.push(MemoryBlock::Free(remaining_part));
                }
            }
            _ => unreachable!(),
        }

        // Copy only the actual data length to the buffer
        self.buffer[start..start + data_len].copy_from_slice(&data); // Ensure only relevant data is copied
        self.blocks.sort_by_key(|block| block.start());
        self.next_id += 1;
        Ok(id)
    }

    pub fn delete(&mut self, id: usize) -> Result<(), String> {
        let block_index = self.blocks.iter()
            .position(|block| match block {
                MemoryBlock::Allocated(allocated_block) => allocated_block.id == id,
                _ => false,
            })
            .ok_or("Block not found".to_string())?;

        let data_index = match &self.blocks[block_index] {
            MemoryBlock::Allocated(block) => block.data_index,
            _ => unreachable!(),
        };

        self.data_storage.remove(data_index);

        for block in self.blocks.iter_mut() {
            if let MemoryBlock::Allocated(allocated_block) = block {
                if allocated_block.data_index > data_index {
                    allocated_block.data_index -= 1;
                }
            }
        }

        let (start, end) = match &self.blocks[block_index] {
            MemoryBlock::Allocated(block) => (block.start, block.end),
            _ => unreachable!(),
        };

        self.allocator.deallocate(start, end - start)
            .map_err(|e| e.to_string())?;

        self.blocks.remove(block_index);
        let free_block = MemoryBlock::Free(FreeBlock::new(start, end));
        self.blocks.push(free_block);
        self.merge_free_blocks();
        self.blocks.sort_by_key(|block| block.start());
        Ok(())
    }

    pub fn update(&mut self, id: usize, data: Vec<u8>) -> Result<(), String> {
        let block_index = self.blocks.iter()
            .position(|block| match block {
                MemoryBlock::Allocated(allocated_block) => allocated_block.id == id,
                _ => false,
            })
            .ok_or("Block not found".to_string())?;

        let (current_start, current_end) = match &self.blocks[block_index] {
            MemoryBlock::Allocated(block) => (block.start, block.end),
            _ => unreachable!(),
        };

        if data.len() > current_end - current_start {
            // If the new data is larger, delete the old block and insert a new one
            self.delete(id)?;
            self.insert(data.len(), data)?;
        } else {
            // Update the existing buffer and data_storage
            self.buffer[current_start..current_start + data.len()].copy_from_slice(&data); // Copy new data to buffer
            let data_index = match &self.blocks[block_index] {
                MemoryBlock::Allocated(block) => block.data_index,
                _ => unreachable!(),
            };

            // Update the data_storage with the new data
            let (mut padded_data, _) = self.data_storage[data_index].clone(); // Get the current data and its length
            padded_data[..data.len()].copy_from_slice(&data); // Update the data
            self.data_storage[data_index] = (padded_data, data.len()); // Store updated data
        }

        Ok(())
    }

    pub fn find(&self, id: usize) -> Result<&AllocatedBlock, String> {
        self.blocks.iter()
            .find_map(|block| match block {
                MemoryBlock::Allocated(allocated_block) if allocated_block.id == id => Some(allocated_block),
                _ => None,
            })
            .ok_or("Block not found".to_string())
    }

    pub fn get_data(&self, block: &AllocatedBlock) -> &[u8] {
        let data_index = block.data_index;
        let (data, actual_size) = &self.data_storage[data_index];
        &data[..*actual_size] // Return actual data without trailing zeros
    }

    pub fn dump(&self) {
        println!("Memory Manager Dump:");
        
        for block in self.blocks.iter() {
            match block {
                MemoryBlock::Free(free_block) => {
                    let mut current_start = free_block.start;
                    while current_start < free_block.end {
                        let block_size = MemoryManager::next_power_of_two_size(free_block.end - current_start); // Use associated function syntax
                        let block_end = current_start + block_size;
                        if block_end > free_block.end {
                            // If the block size would exceed the remaining free space, use the remaining space
                            println!(
                                "0x{:04X} - 0x{:04X}: FREE (Size: {} bytes)",
                                current_start, free_block.end, free_block.end - current_start
                            );
                            break;
                        } else {
                            println!(
                                "0x{:04X} - 0x{:04X}: FREE (Size: {} bytes)",
                                current_start, block_end, block_size
                            );
                        }
                        current_start = block_end;
                    }
                }
                MemoryBlock::Allocated(allocated_block) => {
                    let data = self.get_data(allocated_block);
                    println!(
                        "0x{:04X} - 0x{:04X}: ALLOCATED (ID: {}) (Size: {} bytes)",
                        allocated_block.start, allocated_block.end, allocated_block.id, allocated_block.size()
                    );
                    println!("Data: {:?}", String::from_utf8_lossy(data));
                }
            }
        }
    }

    fn round_up_to_power_of_two(size: usize) -> usize {
        let mut power = 1;
        while power < size {
            power <<= 1;
        }
        power
    }

    fn next_power_of_two_size(size: usize) -> usize {
        // Start with the smallest power of two (16 bytes)
        let mut power = 16;
        while power * 2 <= size {
            power *= 2;
        }
        power
    }

    fn merge_free_blocks(&mut self) {
        let mut merged = false;
        while !merged {
            merged = true;
            for i in 0..self.blocks.len() {
                if let MemoryBlock::Free(current_free) = &self.blocks[i] {
                    for j in (i + 1)..self.blocks.len() {
                        if let MemoryBlock::Free(next_free) = &self.blocks[j] {
                            if current_free.end == next_free.start {
                                let merged_free_block = MemoryBlock::Free(FreeBlock::new(
                                    current_free.start, 
                                    next_free.end
                                ));
                                self.blocks.remove(j);
                                self.blocks[i] = merged_free_block;
                                merged = false;
                                break;
                            }
                        }
                    }
                }
                if !merged {
                    break;
                }
            }
        }
    }
}