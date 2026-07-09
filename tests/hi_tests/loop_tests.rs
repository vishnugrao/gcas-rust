#[cfg(test)]
mod tests {

    // External crates
    use std::io::Read;

    // Internal crates
    use gcas_rust::backend::mem::MemStore;
    use gcas_rust::id::ContentId;
    use gcas_rust::contract::ContentStore;
    
    struct ChunkedReader<'a> {
        data: &'a [u8],
        chunk_size: usize,
    }

    impl<'a> Read for ChunkedReader<'a> {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
            let n = self.chunk_size.min(buf.len()).min(self.data.len());
            buf[..n].copy_from_slice(&self.data[..n]);
            self.data = &self.data[n..];
            Ok(n)
        }
    }

    #[test]
    fn many_small_same_one() {
        let data: Vec<u8> = (0..10_000).map(|i| (i % 256) as u8).collect();
        let expected_id = ContentId::from_bytes(*blake3::hash(&data).as_bytes());

        let store = MemStore::new();
        let mut reader = ChunkedReader {data: &data, chunk_size: 7};
        let result_id = store.put_reader(&mut reader).unwrap();

        assert_eq!(expected_id, result_id);
    }
}
