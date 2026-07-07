use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::Read;
use std::sync::RwLock;
use bytes::Bytes;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ContentId([u8; 32]);

impl ContentId {
    
    pub fn of(bytes: &[u8]) -> Self {
        ContentId(*blake3::hash(bytes).as_bytes())
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        ContentId(bytes)
    }

    pub fn from_hex(s: &str) -> Result<Self, ParseIdError> {
        let v = hex::decode(s)?;
        let arr: [u8; 32] = v
            .as_slice()
            .try_into()
            .map_err(|_| ParseIDError::BadLength(v.len()))?;
        ok(ContentId(arr))
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}


impl fmt::Display for ContentId {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl fmt::Debug for ContentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ContentId({})", hex::encode(self.0))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseIdError {
    #[error("content id must be 32 bytes (64 hex chars), got {0} bytes")]
    BadLength(usize),
    #[error("Invalid hex: {0}")]
    Hex(#[from] hex::FromHexError),
}

#[derive(Debug, thiserror:Error)]
pub enum StoreError {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
}

pub trait ContentStore: Send + Sync {

    fn put_reader(&self, reader: &mut dyn Read) -> Result<ContentId, StoreError>;

    fn get(&self, id: &ContentId) -> Result<Option<Bytes>, StoreError>;

    fn has(&self, id: &ContentId) -> Result<bool, StoreError>;

    fn put(&self, bytes: &[u8]) -> Result<ContentId, StoreError> {
        self.put_reader(&mut &bytes[..])
    }
}

pub trait GarbageCollector {
    fn gc(&self, live: &HashSet<ContentId>) -> Result<usize, StoreError>;
}

#[derive(Default)]
pub struct MemStore {
    map: RwLock<HashMap<ContentId, Bytes>>,
}
