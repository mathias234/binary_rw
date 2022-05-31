//! Stream for operating on in-memory buffers.
use crate::{Stream, BinaryError, Result};
use std::io::{Error, ErrorKind, Read, Write};

/// Stream that wraps an in-memory buffer.
pub struct MemoryStream {
    buffer: Vec<u8>,
    position: usize,
}

impl MemoryStream {
    /// Create a memory stream.
    pub fn new() -> Self {
        MemoryStream {
            buffer: Vec::new(),
            position: 0,
        }
    }
}

impl Stream for MemoryStream {
    fn seek(&mut self, to: usize) -> Result<usize> {
        self.position = to;
        Ok(self.position)
    }

    fn tell(&mut self) -> Result<usize> {
        Ok(self.position)
    }

    fn len(&self) -> Result<usize> {
        Ok(self.buffer.len())
    }
}

impl Read for MemoryStream {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        if self.position + buffer.len() > self.buffer.len() {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                BinaryError::ReadPastEof,
            ));
        }

        let mut idx = 0;
        for i in self.position..self.position + buffer.len() {
            buffer[idx] = self.buffer[i];
            idx += 1;
        }

        self.position += buffer.len();

        Ok(buffer.len())
    }
}

impl Write for MemoryStream {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let bytes_to_end = self.buffer.len() - self.position;
        if bytes.len() > bytes_to_end {
            let bytes_out_of_buffer = bytes.len() - bytes_to_end;

            for _ in 0..bytes_out_of_buffer {
                self.buffer.push(0);
            }
        }

        let mut idx = 0;
        for i in self.position..self.position + bytes.len() {
            self.buffer[i] = bytes[idx];
            idx += 1;
        }

        self.position += bytes.len();

        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl From<Vec<u8>> for MemoryStream {
    fn from(buffer: Vec<u8>) -> Self {
        MemoryStream {
            buffer,
            position: 0,
        }
    }
}

impl Into<Vec<u8>> for MemoryStream {
    fn into(self) -> Vec<u8> {
        self.buffer
    }
}
