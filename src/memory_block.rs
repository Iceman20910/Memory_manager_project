/// Represents a memory block with start and end addresses and associated data.
#[derive(Debug, PartialEq)]
pub struct MemoryBlock {
    pub start: usize,
    pub end: usize,
    pub data: Vec<u8>,
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
