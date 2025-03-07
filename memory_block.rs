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
}