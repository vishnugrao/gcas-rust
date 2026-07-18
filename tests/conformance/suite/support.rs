use std::io::{self, Read};

use bytes::Bytes;
use tempfile::TempDir;

use gcas_rust::contract::ContentStore;
use gcas_rust::error::StoreError;
use gcas_rust::id::ContentId;

pub struct Fixture<S> {
    store: S,
    _dir: Option<TempDir>,
}

impl<S> Fixture<S> {
    pub fn bare(store: S) -> Self {
        Self { store, _dir: None }
    }

    pub fn with_dir(store: S, dir: TempDir) -> Self {
        Self { store, _dir: Some(dir) }
    }
}

impl<S: ContentStore> ContentStore for Fixture<S> {
    fn put_reader(&self, reader: &mut dyn Read) -> Result<ContentId, StoreError> {
        self.store.put_reader(reader)
    }

    fn get(&self, id: &ContentId) -> Result<Option<Bytes>, StoreError> {
        self.store.get(id)
    }

    fn has(&self, id: &ContentId) -> Result<bool, StoreError> {
        self.store.has(id)
    }
}

pub(super) fn ramp(size: usize) -> Vec<u8> {
    (0..size).map(|i| (i % 256) as u8).collect()
}

pub(super) struct ChunkedReader<'a> {
    pub data: &'a [u8],
    pub chunk_size: usize,
}

impl Read for ChunkedReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.chunk_size.min(buf.len()).min(self.data.len());
        buf[..n].copy_from_slice(&self.data[..n]);
        self.data = &self.data[n..];
        Ok(n)
    }
}

pub(super) struct InterruptingReader<'a> {
    pub data: &'a [u8],
    pub chunk_size: usize,
    pub armed: bool,
}

impl Read for InterruptingReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.armed {
            self.armed = false;
            return Err(io::Error::new(io::ErrorKind::Interrupted, "EINTR"));
        }
        self.armed = true;
        let n = self.chunk_size.min(buf.len()).min(self.data.len());
        buf[..n].copy_from_slice(&self.data[..n]);
        self.data = &self.data[n..];
        Ok(n)
    }
}

pub(super) struct FailingReader;

impl Read for FailingReader {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::Other, "boom"))
    }
}

pub(super) struct PartialThenFailReader {
    pub sent: bool,
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
