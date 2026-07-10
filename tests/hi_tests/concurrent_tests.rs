#[cfg(test)]
mod tests {

    // Concurrent puts of the same blob: every thread agrees on the address and
    // dedup collapses to a single entry. Exercises &self + interior locking + Arc.

    use std::sync::Arc;
    use std::thread;

    use gcas_rust::backend::mem::MemStore;
    use gcas_rust::id::ContentId;
    use gcas_rust::contract::ContentStore;

    #[test]
    fn concurrent_same_blob_dedups() {
        let data: Vec<u8> = (0..8192).map(|i| (i % 256) as u8).collect();
        let expected_id = ContentId::of(&data);

        let store = Arc::new(MemStore::new());
        let handles: Vec<_> = (0..16)
            .map(|_| {
                let store = Arc::clone(&store);
                let data = data.clone();
                thread::spawn(move || store.put(&data).unwrap())
            })
            .collect();

        for handle in handles {
            assert_eq!(handle.join().unwrap(), expected_id);
        }

        assert_eq!(store.len(), 1);
    }
}
