#[cfg(test)]
mod tests {
    use crate::MemoryManager; // Import MemoryManager
    use crate::buddy_allocator::BuddyAllocator; // Import BuddyAllocator

    #[test]
    fn test_memory_manager() {
        let mut manager = MemoryManager::new();
        let id = manager.insert(5, b"Hello".to_vec()).unwrap();

        // Compare MemoryBlock's data with byte array
        let block = manager.find(id).unwrap();
        assert_eq!(block.data.as_slice(), b"Hello");

        // Update the memory block
        manager.update(id, b"Goodbye".to_vec()).unwrap();
        let block = manager.find(id).unwrap();
        assert_eq!(block.data.as_slice(), b"Goodbye");

        // Allocation test
        let mut allocator = BuddyAllocator::new();
        let allocated_start = allocator.allocate(10).expect("Allocation failed");
        assert_eq!(allocated_start, 0); // Assuming allocation starts at 0
        allocator.deallocate(allocated_start).expect("Deallocation failed");
    }
}