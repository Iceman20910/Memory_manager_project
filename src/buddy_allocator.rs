pub struct BuddyAllocator {
    free_blocks: Vec<(usize, usize)>, // (start, end)
}

impl BuddyAllocator {
    pub fn new() -> Self {
        let free_blocks = vec![(0, 65536)];
        BuddyAllocator { free_blocks }
    }

    // BS: see https://en.wikipedia.org/wiki/Buddy_memory_allocation
    //
    // You're currently cutting spaces off of a large free block to perform allocation.
    // Instead, you should be cutting the free block in half until it fits the size.
    //
    // Find the first free block that is greater than or equal to the requested size.
    // If the  block is equal to the size, allocate it and you're done.
    // If the block is greater than the requested size, cut it in half until it meets
    // the size requirement.
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