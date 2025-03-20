/// Represents a memory block with start and end addresses and associated data.
#[derive(Debug, PartialEq)]
pub struct MemoryBlock {
    pub start: usize,
    pub end: usize,
    pub data: Vec<u8>,
}

impl MemoryBlock {
    /// Creates a new MemoryBlock instance.
    ///
    /// * `start`: The starting address of the block.
    /// * `end`: The ending address of the block.
    /// * `data`: The data to be stored in the block.
    pub fn new(start: usize, end: usize, data: Vec<u8>) -> Self {
        MemoryBlock { start, end, data }
    }

    /// Sets the data for the block.
    ///
    /// * `data`: The new data to be stored in the block.
    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    /// Gets the data stored in the block.
    ///
    /// * Returns: A reference to the data stored in the block.
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