/// Represents a memory block with start and end addresses
#[derive(Debug, PartialEq)]
pub struct MemoryBlock {
    pub start: usize,
    pub end: usize,
}

impl MemoryBlock {
    pub fn new(start: usize, end: usize) -> Self {
        MemoryBlock { start, end }
    }

    pub fn size(&self) -> usize {
        self.end - self.start
    }
}

impl std::fmt::Display for MemoryBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "0x{:04X} - 0x{:04X}: ALLOCATED (Size: {} bytes)",
            self.start,
            self.end,
            self.end - self.start
        )
    }
}