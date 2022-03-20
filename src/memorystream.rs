use crate::{Stream, StreamError};
use std::io::{Error, ErrorKind, Read, Write};

pub struct Memorystream {
    buffer: Vec<u8>,
    position: usize,
}

impl Memorystream {
    pub fn new() -> Result<Memorystream, StreamError> {
        Ok(Memorystream {
            buffer: Vec::new(),
            position: 0,
        })
    }
}

impl Stream for Memorystream {
    fn seek(&mut self, to: usize) -> Result<usize, StreamError> {
        self.position = to;
        Ok(self.position)
    }

    fn tell(&mut self) -> Result<usize, StreamError> {
        Ok(self.position)
    }
}

impl Read for Memorystream {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        if self.position + buffer.len() > self.buffer.len() {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                StreamError::ReadPastEof,
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

impl Write for Memorystream {
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

impl Into<Vec<u8>> for Memorystream {
    fn into(self) -> Vec<u8> {
        self.buffer
    }
}
