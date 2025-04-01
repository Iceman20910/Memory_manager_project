/// Represents an allocated memory block
#[derive(Debug, Clone)]
pub struct AllocatedBlock {
    pub id: usize,
    pub start: usize,
    pub end: usize,
    pub data_index: usize, // New field to keep track of the index in data_storage
}

impl AllocatedBlock {
    pub fn new(id: usize, start: usize, end: usize, data_index: usize) -> Self {
        AllocatedBlock { id, start, end, data_index }
    }

    pub fn size(&self) -> usize {
        self.end - self.start
    }
}

impl std::fmt::Display for AllocatedBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Allocated Block ID {}: 0x{:04X} - 0x{:04X} (Size: {} bytes, Data Index: {})",
            self.id,
            self.start,
            self.end,
            self.size(),
            self.data_index // Include data_index in the display output
        )
    }
}