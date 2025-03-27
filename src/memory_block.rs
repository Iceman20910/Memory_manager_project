/// Represents a memory block with start and end addresses and associated data.
#[derive(Debug, PartialEq)]
pub struct MemoryBlock {
    pub start: usize,
    pub end: usize,
    // id: usize, // freeblocks do not have ids
    // pub data: Vec<u8>,
}

impl MemoryBlock {
    pub fn new(start: usize, end: usize, data: Vec<u8>) -> Self {
        MemoryBlock { start, end, data }
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
}

// you don't seem to be using this
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
