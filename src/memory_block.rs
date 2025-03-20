/// Represents a memory block with start and end addresses.
#[derive(Debug, PartialEq)]
pub struct MemoryBlock {
    pub start: usize,
    pub end: usize,
}

impl MemoryBlock {
    /// Creates a new MemoryBlock instance.
    ///
    /// * `start`: The starting address of the block.
    /// * `end`: The ending address of the block.
    pub fn new(start: usize, end: usize) -> Self {
        MemoryBlock { start, end }
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