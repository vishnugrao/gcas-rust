// External crates
use tempfile::tempdir;
/* use std::fs;
use std::sync::Arc;
use std::thread;
use std::io::Read; */

// Internal crates
use gcas_rust::backend::loose::LooseStore; 
use gcas_rust::contract::ContentStore;
use gcas_rust::id::ContentId;

#[test]
fn open_empty_dir_is_empty() {
    let dir = tempdir().unwrap();
    let store = LooseStore::open(dir.path()).unwrap();
    assert!(store.is_empty(), "Store is not empty.");
    assert!(dir.path().join("objects").is_dir(), "Objects directory does not exist");
}

#[test]
fn put_small_blob_returns_correct_id() {
    let dir = tempdir().unwrap();
    let store = LooseStore::open(dir.path()).unwrap();
    
    // Expected
    let expected_id: ContentId = ContentId::of(b"hello world!");

    // Returned
    let returned_id: ContentId = store.put(b"hello world!").unwrap();
    assert!(!store.is_empty()); // Basic check to see if the store was written to at all

    assert_eq!(expected_id, returned_id);
}

#[test]
fn get_returns_same_bytes() {
    let dir = tempdir().unwrap();
    let store = LooseStore::open(dir.path()).unwrap();

    let id = store.put(b"hello world!").unwrap();
    assert_eq!(store.get(&id).unwrap().as_deref(), Some(&b"hello world!"[..]));

    let absent = ContentId::of(b"never stored...");
    assert!(store.get(&absent).unwrap().is_none());
}
