#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_manager() {
        let mut manager = MemoryManager::new();
        let id = manager.insert(5, b"Hello".to_vec()).unwrap();

        // Compare MemoryBlock's data with byte array
        let block = manager.find(id).unwrap();
        assert_eq!(block.get_data(), b"Hello");

        // Update the memory block
        manager.update(id, b"Goodbye".to_vec()).unwrap();
        let block = manager.find(id).unwrap();
        assert_eq!(block.get_data(), b"Goodbye");

        // Allocation test
        let mut allocator = BuddyAllocator::new();
        let allocated_start = allocator.allocate(10).expect("Allocation failed");
        assert_eq!(allocated_start, 0); // Assuming allocation starts at 0
        allocator.deallocate(allocated_start, 10).expect("Deallocation failed");
    }

    #[test]
    fn test_command_memory_manager() {
        let mut manager = MemoryManager::new();
        let insert_result = manager.insert(10, b"HelloWorld".to_vec());
        assert!(insert_result.is_ok());
        let id = insert_result.unwrap();

        let read_result = manager.find(id);
        assert!(read_result.is_ok());

        let update_result = manager.update(id, b"ModifiedData".to_vec());
        assert!(update_result.is_ok());

        let delete_result = manager.delete(id);
        assert!(delete_result.is_ok());

        // Check if deleted ID is invalid
        assert!(manager.find(id).is_err());
    }
}