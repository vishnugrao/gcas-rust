use std::io::{self, Read};

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
