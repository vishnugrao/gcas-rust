// External crates
use std::path::{Path, PathBuf};
use std::io::{Read, Write, ErrorKind};
use bytes::Bytes;
use std::fs;
use blake3::Hasher;
use tempfile::NamedTempFile;
use std::ffi::OsStr;

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

    pub fn is_empty(&self) -> Result<bool, StoreError> {
        for shard in fs::read_dir(&self.root)? {
            let shard = shard?;
            if !shard.file_type()?.is_dir() {
                continue;
            }
            let shard_name = shard.file_name();
            for entry in fs::read_dir(shard.path())? {
                let entry = entry?;
                if !entry.file_type()?.is_file() {
                    continue;
                }
                if Self::parse_object_name(&shard_name, &entry.file_name()).is_some() {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    pub fn len(&self) -> Result<usize, StoreError> {
        let mut count = 0;
        for shard in fs::read_dir(&self.root)? {
            let shard = shard?;
            if !shard.file_type()?.is_dir() {
                continue;
            }
            let shard_name = shard.file_name();
            for object in fs::read_dir(shard.path())? {
                let object = object?;
                if !object.file_type()?.is_file() {
                    continue;
                }
                if Self::parse_object_name(&shard_name, &object.file_name()).is_some() {
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    pub fn object_path(&self, id: &ContentId) -> PathBuf {
        let hex = id.to_string();
        self.root.join(&hex[..2]).join(&hex[2..])
    }

    fn parse_object_name(shard: &OsStr, file: &OsStr) -> Option<ContentId> {
        let shard = shard.to_str()?;
        let file = file.to_str()?;
        ContentId::from_hex(&format!("{shard}{file}")).ok()
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
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
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
        match fs::symlink_metadata(self.object_path(id)) {
            Ok(m) => Ok(m.is_file()),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(false),
            Err(e) => Err(StoreError::Io(e)),
        }
    }
} 
