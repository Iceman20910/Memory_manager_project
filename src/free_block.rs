#[derive(Debug, Clone)]
pub struct FreeBlock {
    pub start: usize,
    pub end: usize,
}

impl FreeBlock {
    pub fn new(start: usize, end: usize) -> Self {
        FreeBlock { start, end }
    }

    pub fn size(&self) -> usize {
        self.end - self.start
    }

    // Method to split a free block into two smaller free blocks
    pub fn split(&self, allocation_size: usize) -> (FreeBlock, FreeBlock) {
        let new_start = self.start + allocation_size;
        let _remaining_size = self.size() - allocation_size; // Prefix with underscore to avoid warnings

        (
            FreeBlock::new(self.start, new_start),
            FreeBlock::new(new_start, self.end),
        )
    }
}