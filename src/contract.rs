use std::collections::HashSet;
use std::io::Read;
use bytes::Bytes;

pub trait ContentStore: Send + Sync {

    fn put_reader(&self, reader: &mut dyn Read) -> Result<ContentId, StoreError>;

    fn get(&self, id: &ContentId) -> Result<Option<Bytes>, StoreError>;

    fn has(&self, id: &ContentId) -> Result<bool, StoreError>;

    fn put(&self, bytes: &[u8]) -> Result<ContentId, StoreError> {
        self.put_reader(&mut &bytes[..])
    }
}

pub trait GarbageCollect {
    fn gc(&self, live: &HashSet<ContentId>) -> Result<usize, StoreError>;
}

