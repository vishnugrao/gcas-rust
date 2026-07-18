// External crates
use std::collections::HashMap;
use std::io::{Read, ErrorKind};
use std::sync::RwLock;
use bytes::{Bytes, BytesMut};
use blake3::Hasher;

// Internal crates
use crate::id::ContentId;
use crate::error::StoreError;
use crate::contract::ContentStore;

// Magic numbers
const BUF_SIZE : usize = 64 * 1024;

#[derive(Default)]
pub struct MemStore {
    map: RwLock<HashMap<ContentId, Bytes>>,
}

impl MemStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.map.read().map(|m| m.is_empty()).unwrap_or(false)
    }

    pub fn len(&self) -> usize {
        self.map.read().map(|m| m.len()).unwrap_or(0)
    }
}

impl ContentStore for MemStore {

    fn put_reader(&self, reader: &mut dyn Read) -> Result<ContentId, StoreError> {
        let mut buf = [0u8; BUF_SIZE];            
        let mut hasher = Hasher::new();
        let mut data_accumulator = BytesMut::new();

        loop {
            match reader.read(&mut buf) {
                Ok(0) => break, 
                Ok(n) => {
                    let read_bytes = &buf[..n];

                    hasher.update(read_bytes);
                    data_accumulator.extend_from_slice(read_bytes);
                }
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(StoreError::Io(e)),
            }
        }

        let hash = hasher.finalize();
        let content_id = ContentId::from_bytes(*hash.as_bytes());
        let content_bytes = data_accumulator.freeze();
        let mut map_lock = self.map.write().map_err(|_| StoreError::Poison)?;
        map_lock.insert(content_id, content_bytes);

        Ok(content_id)
    }

    fn get(&self, id: &ContentId) -> Result<Option<Bytes>, StoreError> {
        let map_lock = self.map.read().map_err(|_| StoreError::Poison)?;
        Ok(map_lock.get(id).cloned())
    }

    fn has(&self, id: &ContentId) -> Result<bool, StoreError> {
        let map_lock = self.map.read().map_err(|_| StoreError::Poison)?;
        Ok(map_lock.contains_key(id))
    }
}
