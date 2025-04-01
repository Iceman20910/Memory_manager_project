#[cfg(test)]
mod tests {
    use memory_manager::{MemoryManager, BuddyAllocator};

    #[test]
    fn test_memory_manager_initialization() {
        let memory_manager = MemoryManager::new();
        let buffer = memory_manager.get_buffer();
        
        assert_eq!(buffer.len(), 65536, "Buffer should be initialized with 65536 bytes");
        assert!(buffer.iter().all(|&x| x == 0), "Buffer should be initialized with zeros");
    }

    #[test]
    fn test_memory_insertion_and_retrieval() {
        let mut memory_manager = MemoryManager::new();
        
        // Test inserting multiple blocks
        let data1 = vec![1, 2, 3, 4, 5];
        let id1 = memory_manager.insert(5, data1.clone()).expect("First insertion should succeed");
        
        let data2 = vec![6, 7, 8, 9, 10];
        let id2 = memory_manager.insert(5, data2.clone()).expect("Second insertion should succeed");
        
        // Verify first block
        let block1 = memory_manager.find(id1).expect("First block should be found");
        let retrieved_data1 = memory_manager.get_buffer_slice(block1.start, block1.end);
        assert_eq!(retrieved_data1, &data1, "First block data should match");
        
        // Verify second block
        let block2 = memory_manager.find(id2).expect("Second block should be found");
        let retrieved_data2 = memory_manager.get_buffer_slice(block2.start, block2.end);
        assert_eq!(retrieved_data2, &data2, "Second block data should match");
    }

    #[test]
    fn test_memory_update_and_deletion() {
        let mut memory_manager = MemoryManager::new();
        
        // Insert initial block
        let initial_data = vec![1, 2, 3, 4, 5];
        let id = memory_manager.insert(5, initial_data.clone()).expect("Insertion should succeed");
        
        // Update block with new data
        let updated_data = vec![10, 20, 30, 40, 50];
        memory_manager.update(id, updated_data.clone()).expect("Update should succeed");
        
        // Verify updated block
        let block = memory_manager.find(id).expect("Block should be found after update");
        let retrieved_data = memory_manager.get_buffer_slice(block.start, block.end);
        assert_eq!(retrieved_data, &updated_data, "Block data should be updated");
        
        // Delete block
        memory_manager.delete(id).expect("Deletion should succeed");
        
        // Verify block is no longer findable
        assert!(memory_manager.find(id).is_err(), "Block should not be findable after deletion");
    }

    #[test]
    fn test_memory_allocation_strategy() {
        let mut memory_manager = MemoryManager::new();
        
        // Fill memory with multiple blocks
        let block_sizes = vec![100, 200, 300, 400, 500];
        let mut block_ids = Vec::new();
        
        for &size in &block_sizes {
            let data = vec![0; size];
            let id = memory_manager.insert(size, data).expect("Insertion should succeed");
            block_ids.push(id);
        }
        
        // Delete some blocks
        memory_manager.delete(block_ids[1]).expect("Deletion should succeed");
        memory_manager.delete(block_ids[3]).expect("Deletion should succeed");
        
        // Verify free block merging
        memory_manager.dump(); // This will help visualize the memory state
    }

    #[test]
    fn test_buddy_allocator_integration() {
        let mut allocator = BuddyAllocator::new();
        
        // Allocate blocks of various sizes
        let block1 = allocator.allocate(100).expect("First allocation should succeed");
        let block2 = allocator.allocate(200).expect("Second allocation should succeed");
        
        assert_ne!(block1, block2, "Allocations should be at different addresses");
        
        // Deallocate and reallocate
        allocator.deallocate(block1, 100).expect("First deallocation should succeed");
        let block3 = allocator.allocate(50).expect("Reallocation should succeed");
        
        assert!(block3 <= block1, "Reallocation should use previously freed space");
    }
}
