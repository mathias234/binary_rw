use crate::{Stream, StreamError};

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
    fn write(&mut self, bytes: &Vec<u8>) -> Result<usize, StreamError> {
        let bytes_out_of_buffer = bytes.len() - (self.buffer.len() - self.position);

        for _ in 0..bytes_out_of_buffer {
            self.buffer.push(0);
        }

        let mut idx = 0;
        for i in self.position..self.position + bytes.len() {
            self.buffer[i] = bytes[idx];
            idx += 1;
        }

        self.position += bytes.len();

        Ok(bytes.len())
    }

    fn read(&mut self, buffer: &mut Vec<u8>) -> Result<usize, StreamError> {
        if self.position + buffer.len() > self.buffer.len() {
            return Err(StreamError::ReadError);
        }

        let mut idx = 0;
        for i in self.position..self.position + buffer.len() {
            buffer[idx] = self.buffer[i];
            idx += 1;
        }

        self.position += buffer.len();

        Ok(buffer.len())
    }

    fn seek(&mut self, to: usize) -> Result<usize, StreamError> {
        self.position = to;
        Ok(self.position)
    }

    fn tell(&mut self) -> Result<usize, StreamError> {
        Ok(self.position)
    }
}
