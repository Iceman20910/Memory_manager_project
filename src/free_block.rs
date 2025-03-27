#[derive(Clone)]
pub struct FreeBlock {
    pub start: usize,
    pub end: usize,
}

impl FreeBlock {
    pub fn size(&self) -> usize {
        self.end - self.start
    }
}

impl std::fmt::Display for FreeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Free Block: 0x{:04X} - 0x{:04X} (Size: {} bytes)",
            self.start,
            self.end,
            self.size()
        )
    }
}
