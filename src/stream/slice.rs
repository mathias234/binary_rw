//! Stream that reads from a slice of bytes.
use crate::{BinaryError, ReadStream, Result, SeekStream};
use std::io::{Error, ErrorKind, Read};

/// Stream that wraps a slice of bytes.
pub struct SliceStream<'a> {
    buffer: &'a [u8],
    position: usize,
}

impl<'a> SliceStream<'a> {
    /// Create a slice stream.
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            position: 0,
        }
    }
}

impl SeekStream for SliceStream<'_> {
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

impl Read for SliceStream<'_> {
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

impl ReadStream for SliceStream<'_> {}
