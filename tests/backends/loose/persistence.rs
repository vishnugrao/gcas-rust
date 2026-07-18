// External crates
use tempfile::tempdir;
use std::io::{self, Read, ErrorKind};
use std::fs;

// Internal crates
use gcas_rust::backend::loose::LooseStore; 
use gcas_rust::contract::ContentStore;
use gcas_rust::id::ContentId;
use gcas_rust::error::StoreError;

struct PartialThenFailReader {
    sent: bool,
}

impl Read for PartialThenFailReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.sent {
            return Err(io::Error::new(io::ErrorKind::BrokenPipe, "pipe broke"));
        }
        self.sent = true;
        let n = buf.len().min(8);
        buf[..n].fill(0xAB);
        Ok(n)
    }
}

#[test]
fn open_empty_dir_is_empty() {
    let dir = tempdir().unwrap();
    let store = LooseStore::open(dir.path()).unwrap();
    assert!(store.is_empty().unwrap(), "Store is not empty.");
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
    assert!(!store.is_empty().unwrap()); // Basic check to see if the store was written to at all

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

#[test]
fn survives_reopen() {
    let dir = tempdir().unwrap();

    let id = {
        let initial_store = LooseStore::open(dir.path()).unwrap();
        initial_store.put(b"hello world!").unwrap()
    };

    let reopened_store = LooseStore::open(dir.path()).unwrap();
    assert_eq!(reopened_store.get(&id).unwrap().as_deref(), Some(&b"hello world!"[..]));
}

#[test]
fn writes_sharded_object_file() {
    let dir = tempdir().unwrap();
    let store = LooseStore::open(dir.path()).unwrap();
    let id = store.put(b"hello world!").unwrap();

    let hex = id.to_string();
    let object_path = dir.path().join("objects").join(&hex[..2]).join(&hex[2..]);
    assert!(object_path.is_file(), "expected sharded object at {object_path:?}");
}

#[test]
fn put_twice_dedups() {
    let dir = tempdir().unwrap();
    let store = LooseStore::open(dir.path()).unwrap();
    let first_id = store.put(b"hello world!").unwrap();
    let second_id = store.put(b"hello world!").unwrap();

    assert_eq!(first_id, second_id);
    assert_eq!(store.len().unwrap(), 1);
}

#[test]
fn interrupted_write_leaves_no_object() {
    let dir = tempdir().unwrap();
    let store = LooseStore::open(dir.path()).unwrap();
    let err = store.put_reader(&mut PartialThenFailReader{sent: false}).unwrap_err();
    assert!(matches!(&err, StoreError::Io(e) if e.kind() == ErrorKind::BrokenPipe), "expected the injected BrokenPipe, got {err:?}");
    assert!(store.is_empty().unwrap(), "Interrupted put left an object behind");
}

#[test]
fn only_files_count_as_objects() {
    let dir = tempdir().unwrap();
    let store = LooseStore::open(dir.path()).unwrap();
    let never_stored_id = ContentId::of(b"never stored...");
    let hex = never_stored_id.to_string();
    let never_stored_dir = dir.path().join("objects").join(&hex[..2]).join(&hex[2..]);
    fs::create_dir_all(never_stored_dir).unwrap();

    assert_eq!(store.len().unwrap(), 0);
    assert!(store.is_empty().unwrap());
    assert!(!store.has(&never_stored_id).unwrap());
}
