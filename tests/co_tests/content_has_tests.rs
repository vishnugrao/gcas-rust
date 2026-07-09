#[cfg(test)]
mod tests {
    
    // Tests for content out
    // Test has with item in map
    // Test has without item in map
    
    use gcas_rust::backend::mem::MemStore;
    use gcas_rust::contract::ContentStore;
    use gcas_rust::id::ContentId;

    fn init_map_with_one() -> (MemStore, ContentId) {
        let store = MemStore::new();
        let data: Vec<u8> = (0..4096).map(|i| (i % 256) as u8).collect();
        let id = store.put_reader(&mut &data[..]).unwrap();
        (store, id)
    }

    #[test]
    fn has_exists() {
        let (store, in_map_id) = init_map_with_one();
        assert!(store.has(&in_map_id).unwrap());
    }

    #[test]
    fn has_not_exists() {
        let (store, _) = init_map_with_one();

        // Straw id
        let data: Vec<u8> = (0..256).map(|i| (i % 256) as u8).collect();
        let rand_id = ContentId::from_bytes(*blake3::hash(&data).as_bytes());
        
        assert!(!store.has(&rand_id).unwrap());
    }
}
