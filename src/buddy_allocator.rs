pub struct BuddyAllocator {
    free_blocks: Vec<(usize, usize)>, // (start, end)
}

impl BuddyAllocator {
    pub fn new() -> Self {
        let free_blocks = vec![(0, 65536)]; // Initialize with a single large free block
        BuddyAllocator { free_blocks }
    }

    // Allocate memory using buddy allocation strategy
    pub fn allocate(&mut self, size: usize) -> Result<usize, String> {
        if size == 0 {
            return Err("Invalid size".to_string());
        }

        let aligned_size = size.next_power_of_two(); // Align size to next power of two

        for i in 0..self.free_blocks.len() {
            let (start, end) = self.free_blocks[i];
            let mut block_size = end - start; // Make this variable mutable

            if block_size >= aligned_size {
                // If the block is larger than the aligned size, split it
                while block_size > aligned_size {
                    block_size /= 2; // Cut the block size in half
                }

                // Remove the original block
                self.free_blocks.remove(i);

                // Add the remaining block if there's space left
                if block_size > 0 {
                    self.free_blocks.push((start + aligned_size, end)); // Add the remaining free block
                }
                return Ok(start); // Return the start address of the allocated block
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

        self.free_blocks.push((start, end)); // Add the freed block
        self.merge_free_blocks(); // Merge adjacent free blocks
        Ok(())
    }

    fn merge_free_blocks(&mut self) {
        self.free_blocks.sort_by_key(|&(start, _)| start); // Sort by starting address
        let mut i = 0;
        while i < self.free_blocks.len() - 1 {
            let (start1, end1) = self.free_blocks[i];
            let (start2, end2) = self.free_blocks[i + 1];
            if end1 == start2 {
                // If two blocks are adjacent, merge them
                self.free_blocks[i] = (start1, end2);
                self.free_blocks.remove(i + 1);
            } else {
                i += 1; // Move to the next block
            }
        }
    }

    pub fn dump_free_blocks(&self) {
        for (start, end) in &self.free_blocks {
            println!("Free Block: 0x{:04X} - 0x{:04X}", start, end);
        }
    }
}