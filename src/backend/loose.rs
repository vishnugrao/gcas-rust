// External crates
use std::path::{Path, PathBuf};
use std::io::{Read, Write, ErrorKind};
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

    pub fn object_path(&self, id: &ContentId) -> PathBuf {
        let hex = id.to_string();
        self.root.join(&hex[..2]).join(&hex[2..])
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
        let final_path = self.object_path(&content_id);
        fs::create_dir_all(final_path.parent().unwrap())?;
        named_temp_file.persist(final_path)?;
        Ok(content_id)
    }    

    fn get(&self, id: &ContentId) -> Result<Option<Bytes>, StoreError> {
        let path = self.object_path(id);
        match fs::read(&path) {
            Ok(data) => Ok(Some(Bytes::from(data))),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
            Err(e) => Err(StoreError::Io(e)),
        }
    }

    fn has(&self, id: &ContentId) -> Result<bool, StoreError> {
        Ok(self.object_path(id).try_exists()?)
    }
} 
