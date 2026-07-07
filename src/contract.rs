use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::Read;
use std::sync::RwLock;
use bytes::Bytes;

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


