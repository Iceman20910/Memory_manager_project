#[cfg(test)]
mod tests {
    use memory_manager::{MemoryManager, BuddyAllocator};

    #[test]
    fn test_memory_manager() {
        let mut manager = MemoryManager::new();
        let id = manager.insert(5, b"Hello".to_vec()).unwrap();

        // Compare data in the buffer with byte array
        let block = manager.find(id).unwrap();
        let data = manager.get_buffer_slice(block.start, block.end);
        assert_eq!(data, b"Hello");

        // Update the memory block
        manager.update(id, b"Goodbye".to_vec()).unwrap();

        // Access updated data in a separate scope
        {
            let block = manager.find(id).unwrap(); // Reborrow block
            let updated_data = manager.get_buffer_slice(block.start, block.end);
            assert_eq!(updated_data, b"Goodbye");
        }

        // Allocation test
        let mut allocator = BuddyAllocator::new();
        let allocated_start = allocator.allocate(10).expect("Allocation failed");
        assert_eq!(allocated_start, 0); // Assuming allocation starts at 0
        allocator
            .deallocate(allocated_start, 10)
            .expect("Deallocation failed");
    }

    #[test]
    fn test_command_memory_manager() {
        let mut manager = MemoryManager::new();
        let insert_result = manager.insert(10, b"HelloWorld".to_vec());
        assert!(insert_result.is_ok());
        let id = insert_result.unwrap();

        let read_result = manager.find(id);
        assert!(read_result.is_ok());

        // Check data after insertion
        {
            let block = read_result.unwrap();
            let data = manager.get_buffer_slice(block.start, block.end);
            assert_eq!(data, b"HelloWorld");
        }

        let update_result = manager.update(id, b"ModifiedData".to_vec());
        assert!(update_result.is_ok());

        // Check data after update in a separate scope
        {
            let block = manager.find(id).unwrap(); // Reborrow block
            let updated_data = manager.get_buffer_slice(block.start, block.end);
            assert_eq!(updated_data, b"ModifiedData");
        }

        let delete_result = manager.delete(id);
        assert!(delete_result.is_ok());

        // Check if deleted ID is invalid
        assert!(manager.find(id).is_err());
    }
}
