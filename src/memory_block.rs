// BS: You should look into what this does (part 1):
// use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct MemoryBlock {
    pub start: usize,
    pub end: usize,

    // BS: this field should be deleted.
    // Memory blocks should not store data, only
    // references to where data is in the buffer.
    pub data: Vec<u8>,
}

impl MemoryBlock {
    pub fn new(start: usize, end: usize, data: Vec<u8>) -> Self {
        MemoryBlock { start, end, data }
    }
}

// BS: You should look into what this does (part 2):
/* impl Display for MemoryBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Block: [{}, {}]", self.start, self.end)
    }
} */

