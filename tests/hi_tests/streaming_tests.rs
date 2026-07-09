#[cfg(test)]
mod tests {
    // Write tests w.r.t. streaming hasher:
    // Single update then finalize
    // Multiple update then finalize
    // Negative: reader I/O errors propagate as StoreError::Io

    use std::io::{self, Read};

    use gcas_rust::backend::mem::MemStore;
    use gcas_rust::contract::ContentStore;
    use gcas_rust::error::StoreError;
    use gcas_rust::id::ContentId;

    /// Reader that fails immediately on the first `read` call.
    struct FailingReader;
    impl Read for FailingReader {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "boom"))
        }
    }

    /// Reader that yields one chunk of data and then fails, exercising the
    /// error path *after* the hasher has already consumed some bytes.
    struct PartialThenFailReader {
        sent: bool,
    }
    impl Read for PartialThenFailReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            if self.sent {
                Err(io::Error::new(io::ErrorKind::BrokenPipe, "pipe broke"))
            } else {
                self.sent = true;
                let n = buf.len().min(8);
                buf[..n].fill(0xAB);
                Ok(n)
            }
        }
    }

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

    #[test]
    fn put_reader_propagates_immediate_io_error() {
        let store = create_in_mem_test_store();
        let err = store.put_reader(&mut FailingReader).unwrap_err();

        assert!(
            matches!(err, StoreError::Io(_)),
            "expected StoreError::Io, got {err:?}"
        );
        // A failed put must not leave anything behind.
        assert!(store.is_empty());
    }

    #[test]
    fn put_reader_propagates_io_error_after_partial_read() {
        let store = create_in_mem_test_store();
        let mut reader = PartialThenFailReader { sent: false };
        let err = store.put_reader(&mut reader).unwrap_err();

        assert!(
            matches!(err, StoreError::Io(_)),
            "expected StoreError::Io, got {err:?}"
        );
        assert!(store.is_empty());
    }

    #[test]
    fn put_delegates_to_put_reader_error_free() {
        // Sanity: the default `put` wrapper hashes identically to `put_reader`.
        let store = create_in_mem_test_store();
        let data: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();

        let expected_id = ContentId::from_bytes(*blake3::hash(&data).as_bytes());
        let id = store.put(&data).unwrap();

        assert_eq!(id, expected_id);
    }
}
