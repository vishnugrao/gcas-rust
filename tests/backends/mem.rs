use std::io::{self, Read};
use std::sync::Arc;
use std::thread;

use gcas_rust::backend::mem::MemStore;
use gcas_rust::contract::ContentStore;

struct FailingReader;

impl Read for FailingReader {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::Other, "boom"))
    }
}

#[test]
fn new_store_is_empty() {
    assert!(MemStore::new().is_empty());
}

#[test]
fn failed_put_stores_nothing() {
    let store = MemStore::new();
    store.put_reader(&mut FailingReader).unwrap_err();
    assert!(store.is_empty());
}

#[test]
fn concurrent_same_blob_dedups_to_one_entry() {
    let data: Vec<u8> = (0..8192).map(|i| (i % 256) as u8).collect();
    let store = Arc::new(MemStore::new());

    let handles: Vec<_> = (0..16)
        .map(|_| {
            let store = Arc::clone(&store);
            let data = data.clone();
            thread::spawn(move || store.put(&data).unwrap())
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
    assert_eq!(store.len(), 1);
}
