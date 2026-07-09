#[cfg(test)]
mod tests {
    
    // Tests for get
    // One test with get exists
    // One test get not exists


    use gcas_rust::backend::mem::MemStore;
    use gcas_rust::id::ContentId;
    use gcas_rust::contract::ContentStore;

    fn init_map_with_one() -> (MemStore, ContentId) {
        let store = MemStore::new();
        let data: Vec<u8> = (0..4096).map(|i| (i % 256) as u8).collect();
        let id = store.put_reader(&mut &data[..]).unwrap();
        (store, id)
    }

    #[test]
    fn get_exists() {
        let (store, in_map_id) = init_map_with_one();
        let data: Vec<u8> = (0..4096).map(|i| (i % 256) as u8).collect();
        let retrieved = store.get(&in_map_id).unwrap().unwrap();
        assert_eq!(data, retrieved.to_vec());
    }

    #[test]
    fn get_not_exists() {
        let (store, _) = init_map_with_one();

        // Straw id
        let data: Vec<u8> = (0..256).map(|i| (i % 256) as u8).collect();
        let rand_id = ContentId::from_bytes(*blake3::hash(&data).as_bytes());

        assert_eq!(None, store.get(&rand_id).unwrap());
    }
}
