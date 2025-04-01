use crate::free_block::FreeBlock;
use crate::allocated_block::AllocatedBlock;
use crate::buddy_allocator::BuddyAllocator;

#[derive(Debug, Clone)]
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
    data_storage: Vec<Vec<u8>>,
    blocks: Vec<MemoryBlock>,
    next_id: usize,
}

impl MemoryManager {
    pub fn new() -> Self {
        let buffer_size = 65536;
        let buffer = vec![0u8; buffer_size];

        // Initially, the entire buffer is a free block
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

    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn get_buffer_slice(&self, start: usize, end: usize) -> &[u8] {
        &self.buffer[start..end]
    }

    pub fn insert(&mut self, size: usize, data: Vec<u8>) -> Result<usize, String> {
        println!("Attempting to insert block of size {} with data {:?}", size, data);
        
        // Use buddy allocator to find a suitable block
        let start = self.allocator.allocate(size).map_err(|e| e.to_string())?;
        let end = start + size;

        // Create allocated block
        let id = self.next_id;
        let padded_data = {
            let mut padded_data = vec![0u8; size];
            let data_len = data.len();
            padded_data[..data_len].copy_from_slice(&data); // Copy original data
            padded_data
        };

        // Store data and get the index
        let data_index = self.data_storage.len();
        self.data_storage.push(padded_data); // Store the padded data

        let allocated_block = MemoryBlock::Allocated(AllocatedBlock::new(id, start, end, data_index));

        // Update blocks list
        let block_index = self.blocks.iter()
            .position(|block| match block {
                MemoryBlock::Free(free_block) => free_block.start <= start && free_block.end >= end,
                _ => false,
            })
            .ok_or("No suitable free block found".to_string())?;

        // Remove the original free block
        let original_block = self.blocks.remove(block_index);

        // Add allocated block
        self.blocks.push(allocated_block);

        // Handle remaining free space
        match original_block {
            MemoryBlock::Free(free_block) => {
                // Add free block before allocation if exists
                if free_block.start < start {
                    let pre_free_block = MemoryBlock::Free(FreeBlock::new(free_block.start, start));
                    self.blocks.push(pre_free_block);
                }

                // Add free block after allocation if exists
                if end < free_block.end {
                    let post_free_block = MemoryBlock::Free(FreeBlock::new(end, free_block.end));
                    self.blocks.push(post_free_block);
                }
            }
            _ => unreachable!(),
        }

        // Copy data to buffer
        self.buffer[start..end].copy_from_slice(&self.data_storage[data_index]);

        // Sort blocks by start address
        self.blocks.sort_by_key(|block| block.start());

        self.next_id += 1;
        Ok(id)
    }

    pub fn delete(&mut self, id: usize) -> Result<(), String> {
        println!("Attempting to delete block ID {}", id);
        
        // Find the block by ID
        let block_index = self.blocks.iter()
            .position(|block| match block {
                MemoryBlock::Allocated(allocated_block) => allocated_block.id == id,
                _ => false,
            })
            .ok_or("Block not found".to_string())?;

        // Get the data index from the block being deleted
        let data_index = match &self.blocks[block_index] {
            MemoryBlock::Allocated(block) => block.data_index,
            _ => unreachable!(),
        };
        
        // Remove the corresponding data from data_storage
        self.data_storage.remove(data_index);

        // Update data indices of remaining blocks
        for block in self.blocks.iter_mut() {
            if let MemoryBlock::Allocated(allocated_block) = block {
                if allocated_block.data_index > data_index {
                    allocated_block.data_index -= 1; // Decrement index for blocks after the deleted block
                }
            }
        }

        // Deallocate memory using buddy allocator
        let (start, end) = match &self.blocks[block_index] {
            MemoryBlock::Allocated(block) => (block.start, block.end),
            _ => unreachable!(),
        };

        self.allocator.deallocate(start, end - start)
            .map_err(|e| e.to_string())?;

        // Remove the block from blocks list
        self.blocks.remove(block_index);

        // Add back as a free block
        let free_block = MemoryBlock::Free(FreeBlock::new(start, end));
        self.blocks.push(free_block);

        // Merge adjacent free blocks
        self.merge_free_blocks();

        // Sort blocks by start address
        self.blocks.sort_by_key(|block| block.start());

        Ok(())
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
                                // Merge adjacent free blocks
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

    pub fn update(&mut self, id: usize, data: Vec<u8>) -> Result<(), String> {
        println!("Attempting to update block ID {} with data {:?}", id, data);
        
        // Find the block by ID
        let block_index = self.blocks.iter()
            .position(|block| match block {
                MemoryBlock::Allocated(allocated_block) => allocated_block.id == id,
                _ => false,
            })
            .ok_or("Block not found".to_string())?;

        // Get current block details
        let (current_start, current_end) = match &self.blocks[block_index] {
            MemoryBlock::Allocated(block) => (block.start, block.end),
            _ => unreachable!(),
        };

        // Check if new data fits in existing block
        if data.len() > current_end - current_start {
            // Need to reallocate
            self.delete(id)?;

            // Insert new data with the correct size
            self.insert(data.len(), data)?;
        } else {
            // Update buffer in-place
            self.buffer[current_start..current_start + data.len()].copy_from_slice(&data);

            // Update data storage using the data index from the block
            let data_index = match &self.blocks[block_index] {
                MemoryBlock::Allocated(block) => block.data_index,
                _ => unreachable!(),
            };
            
            self.data_storage[data_index] = data; // Update with new data
        }

        Ok(())
    }

    pub fn find(&self, id: usize) -> Result<&AllocatedBlock, String> {
        println!("Attempting to find block ID {}", id);
        self.blocks.iter()
            .find_map(|block| match block {
                MemoryBlock::Allocated(allocated_block) if allocated_block.id == id => Some(allocated_block),
                _ => None,
            })
            .ok_or("Block not found".to_string())
    }

    pub fn get_data(&self, block: &AllocatedBlock) -> &[u8] {
        let data_index = block.data_index; // Accessing the data index directly from the block
        &self.data_storage[data_index]
    }

    pub fn dump(&self) {
        println!("Memory Manager Dump:");
        
        for (i, block) in self.blocks.iter().enumerate() {
            match block {
                MemoryBlock::Free(free_block) => {
                    println!(
                        "Block {}: Free Block, Start: 0x{:04X}, End: 0x{:04X}, Size: {}",
                        i, free_block.start, free_block.end, free_block.end - free_block.start
                    );
                }
                MemoryBlock::Allocated(allocated_block) => {
                    let data = self.get_data(allocated_block);
                    println!(
                        "Block {}: Allocated Block ID: {}, Start: 0x{:04X}, End: 0x{:04X}, Size: {}, Data: {:?}",
                        i, allocated_block.id, allocated_block.start, allocated_block.end, allocated_block.end - allocated_block.start, data
                    );
                }
            }
        }
    }
}