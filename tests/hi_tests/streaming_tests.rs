#[cfg(test)]
mod tests {
    // Write tests w.r.t. streaming hasher:
    // Single update then finalize
    // Multiple update then finalize

    use gcas_rust::backend::mem::MemStore;
    use gcas_rust::contract::ContentStore;
    use gcas_rust::id::ContentId;

    fn create_in_mem_test_store() -> MemStore {
        MemStore::new()
    }

    #[test]
    fn try_init() {
        let store = create_in_mem_test_store();
        assert!(store.is_empty());
    }

    #[test]
    fn small_4kb_hash_stable() {
        let data: Vec<u8> = (0..4096).map(|i| (i % 256) as u8).collect();
        
        // Expected
        let expected_id = ContentId::from_bytes(*blake3::hash(&data).as_bytes());

        // Result
        let store = create_in_mem_test_store();
        let result_id: ContentId = store.put_reader(&mut &data[..]).unwrap();
        
        assert_eq!(result_id, expected_id);
    }

    #[test]
    fn medium_hash_boundary_stable() {
        for size in [65535, 65536, 65537] {
            let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
            
            // Expected
            let expected_id = ContentId::from_bytes(*blake3::hash(&data).as_bytes());

            // Result
            let store = create_in_mem_test_store();
            let result_id = store.put_reader(&mut &data[..]).unwrap();

            assert_eq!(result_id, expected_id, "mismatch at size={size}");
        }
    }

    #[test]
    fn large_multi_hash_stable() {
        for size in [3 * 65536, 256 * 65536] {
            let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();

            // Expected
            let expected_id = ContentId::from_bytes(*blake3::hash(&data).as_bytes());

            // Result
            let store = create_in_mem_test_store();
            let result_id = store.put_reader(&mut &data[..]).unwrap();

            assert_eq!(result_id, expected_id, "mismatch at size={size}");
        }
    }
}
