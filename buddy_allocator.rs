use std::sync::{Mutex, Arc};

pub struct BuddyAllocator {
    free_blocks: Arc<Mutex<Vec<(usize, usize)>>>,
}

impl BuddyAllocator {
    pub fn new() -> Self {
        let free_blocks = Arc::new(Mutex::new(vec![(0, 65535)])); // Example range
        BuddyAllocator { free_blocks }
    }

    pub fn allocate(&mut self, size: usize) -> Result<usize, String> {
        if size == 0 {
            return Err("Invalid size".to_string());
        }

        let mut free_blocks = self.free_blocks.lock().unwrap();
        if let Some(index) = free_blocks.iter().position(|&(start, end)| end - start >= size) {
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