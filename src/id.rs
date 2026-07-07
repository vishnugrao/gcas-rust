use bytes::Bytes;
use std::fmt;

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
