use std::sync::{Arc, Mutex};

pub struct BuddyAllocator {
    // BS: you DEFINITELY don't need an Arc Mutex...
    //
    // This is interesting. You have a vector of a single tuple?
    // with the way you implemented this, free_blocks: (usize, usize)
    // would have worked.
    //
    // I think there is a fundamental misunderstanding regarding the
    // way memory manager should work. hopefully the comments below will
    // help...
    free_blocks: Arc<Mutex<Vec<(usize, usize)>>>,
}

impl BuddyAllocator {
    pub fn new() -> Self {
        let free_blocks = Arc::new(Mutex::new(vec![(0, 65535)])); // Example range
        BuddyAllocator { free_blocks }
    }

    // BS: allocation isn't that simple.
    //
    // 1.) If I request an allocation of 5 bytes, then you should allocate
    // 8, the smallest power of two that'll fit it.
    //
    // 2.) if i allocate 8 bytes, that doesnt mean you have a free block of
    // 65535 - 8 bytes. Instead, you actually have an allocated block of 8 bytes,
    // a free block of 8 bytes, another free block of 16 bytes, 32, 64, 128, 256,
    // 512, 1024, 2048, 4096, 8192, 16384, and 32768. This means 13 free blocks.
    // look at this for reference https://en.wikipedia.org/wiki/Buddy_memory_allocation
    pub fn allocate(&mut self, size: usize) -> Result<usize, String> {
        if size == 0 {
            return Err("Invalid size".to_string());
        }

        let mut free_blocks = self.free_blocks.lock().unwrap();
        if let Some(index) = free_blocks
            .iter()
            .position(|&(start, end)| end - start >= size)
        {
            let (start, end) = free_blocks[index];
            free_blocks.remove(index);

            if end - start > size {
                free_blocks.push((start + size, end));
            }

            return Ok(start);
        }

        Err("Insufficient memory".to_string())
    }

    pub fn deallocate(&mut self, start: usize) -> Result<(), String> {
        let mut free_blocks = self.free_blocks.lock().unwrap();
        let index = free_blocks
            .iter()
            .position(|&(block_start, _)| block_start > start)
            .unwrap_or(free_blocks.len());

        free_blocks.insert(index, (start, start + 1));
        // usually, when you need to manually call drop, you're doing
        // something fishy..
        drop(free_blocks);
        self.merge_free_blocks();
        Ok(())
    }

    fn merge_free_blocks(&mut self) {
        let mut free_blocks = self.free_blocks.lock().unwrap();
        free_blocks.sort_by_key(|&(start, _)| start);

        let mut i = 0;
        while i < free_blocks.len() - 1 {
            let (start1, end1) = free_blocks[i];
            let (start2, end2) = free_blocks[i + 1];

            if end1 == start2 {
                free_blocks[i] = (start1, end2);
                free_blocks.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    #[allow(dead_code)] // Suppress the dead code warning
    pub fn dump_free_blocks(&self) {
        let free_blocks = self.free_blocks.lock().unwrap();
        for (start, end) in &*free_blocks {
            println!("Free Block: 0x{:04X} - 0x{:04X}", start, end);
        }
    }
}

