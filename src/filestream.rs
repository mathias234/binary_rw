use crate::{Stream, StreamError};
use std::fs;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;

pub enum OpenType {
    OpenAndCreate,
    Open,
}

pub struct Filestream {
    file: fs::File,
}

impl Filestream {
    pub fn new<P: AsRef<Path>>(path: P, open_type: OpenType) -> Result<Filestream, StreamError> {
        let file = match open_type {
            OpenType::OpenAndCreate => fs::File::create(path)?,
            OpenType::Open => fs::File::open(path)?,
        };
        Ok(Filestream { file })
    }
}

impl Stream for Filestream {
    fn write(&mut self, bytes: &Vec<u8>) -> Result<usize, StreamError> {
        Ok(self.file.write(bytes)?)
    }

    fn read(&mut self, buffer: &mut Vec<u8>) -> Result<usize, StreamError> {
        if self.tell().unwrap() + buffer.len() > self.file.metadata()?.len() as usize {
            return Err(StreamError::ReadPastEof);
        }
        Ok(self.file.read(buffer)?)
    }

    fn seek(&mut self, to: usize) -> Result<usize, StreamError> {
        Ok(self.file.seek(SeekFrom::Start(to as u64))? as usize)
    }

    fn tell(&mut self) -> Result<usize, StreamError> {
        Ok(self.file.seek(SeekFrom::Current(0))? as usize)
    }
}
