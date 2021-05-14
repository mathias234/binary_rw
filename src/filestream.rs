use crate::{Stream, StreamError};
use std::fs;
use std::io::prelude::*;
use std::io::SeekFrom;

pub enum OpenType {
    OpenAndCreate,
    Open,
}

pub struct Filestream {
    file: fs::File,
}

impl Filestream {
    pub fn new(filepath: &str, open_type: OpenType) -> Result<Filestream, StreamError> {
        let file;

        match open_type {
            OpenType::OpenAndCreate => file = fs::File::create(filepath),
            OpenType::Open => file = fs::File::open(filepath),
        }

        match file {
            Ok(f) => Ok(Filestream { file: f }),
            Err(_) => Err(StreamError::OpenError),
        }
    }
}

impl Stream for Filestream {
    fn write(&mut self, bytes: &Vec<u8>) -> Result<u64, StreamError> {
        match self.file.write(bytes) {
            Ok(res) => Ok(res as u64),
            Err(_) => Err(StreamError::WriteError),
        }
    }

    fn read(&mut self, buffer: &mut Vec<u8>) -> Result<u64, StreamError> {
        match self.file.read(buffer) {
            Ok(res) => Ok(res as u64),
            Err(_) => Err(StreamError::ReadError),
        }
    }

    fn seek(&mut self, from: u64) -> Result<u64, StreamError> {
        match self.file.seek(SeekFrom::Start(from)) {
            Ok(res) => Ok(res as u64),
            Err(_) => Err(StreamError::SeekError),
        }
    }

    fn tell(&mut self) -> Result<u64, StreamError> {
        match self.file.seek(SeekFrom::Current(0)) {
            Ok(res) => Ok(res as u64),
            Err(_) => Err(StreamError::TellError),
        }
    }
}
