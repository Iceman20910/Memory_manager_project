/// Represents an allocated memory block
#[derive(Debug, Clone)]
pub struct AllocatedBlock {
    pub id: usize,
    pub start: usize,
    pub end: usize,
}

impl AllocatedBlock {
    pub fn new(id: usize, start: usize, end: usize) -> Self {
        AllocatedBlock { id, start, end }
    }

    pub fn size(&self) -> usize {
        self.end - self.start
    }
}

impl std::fmt::Display for AllocatedBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Allocated Block ID {}: 0x{:04X} - 0x{:04X} (Size: {} bytes)",
            self.id,
            self.start,
            self.end,
            self.size()
        )
    }
}
