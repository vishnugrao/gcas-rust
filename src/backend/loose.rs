// External crates
use std::path::{Path, PathBuf};
use std::io::{Read, Write};
use bytes::Bytes;
use std::fs;
use blake3::Hasher;
use tempfile::NamedTempFile;

// Internal crates
use crate::id::ContentId;
use crate::error::StoreError;
use crate::contract::ContentStore;

// Magic numbers
const BUF_SIZE : usize = 64 * 1024;

pub struct LooseStore {
    root: PathBuf,
} 

impl LooseStore {

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, StoreError> {
        let mut root: PathBuf = path.as_ref().to_path_buf();
        root.push("objects");
        fs::create_dir_all(&root)?;
        Ok(LooseStore{root})
    }

    pub fn is_empty(&self) -> bool {
        fs::read_dir(&self.root)
            .map(|mut entries| entries.next().is_none())
            .unwrap_or(false)
    }

    pub fn len(&self) -> usize {
        /* aggregate counts (maybe additional bookkeeping on insert / del, not sure */
        todo!()
    }
}

impl ContentStore for LooseStore {
    fn put_reader(&self, reader: &mut dyn Read) -> Result<ContentId, StoreError> {
        let mut buf = [0u8; BUF_SIZE];
        let mut hasher = Hasher::new();
        let mut named_temp_file = NamedTempFile::new_in(&self.root)?;
        
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    let read_bytes = &buf[..n];
                    hasher.update(read_bytes);
                    named_temp_file.write_all(read_bytes)?;
                }
                Err(e) => return Err(StoreError::Io(e)),
            }
        }

        let hash = hasher.finalize();
        let content_id = ContentId::from_bytes(*hash.as_bytes());
        named_temp_file.persist(self.root.join(content_id.to_string()))?;
        Ok(content_id)
    }    

    fn get(&self, id: &ContentId) -> Result<Option<Bytes>, StoreError> {
        /* Todo */
        todo!()
    }

    fn has(&self, id: &ContentId) -> Result<bool, StoreError> {
        /* Todo */
        todo!()
    }
} 
