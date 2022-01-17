use crate::{Stream, StreamError};
use std::fs;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Read, SeekFrom, Write};
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
    fn seek(&mut self, to: usize) -> Result<usize, StreamError> {
        Ok(self.file.seek(SeekFrom::Start(to as u64))? as usize)
    }

    fn tell(&mut self) -> Result<usize, StreamError> {
        Ok(self.file.seek(SeekFrom::Current(0))? as usize)
    }
}

impl Read for Filestream {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        if self.tell().unwrap() + buffer.len() > self.file.metadata()?.len() as usize {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                StreamError::ReadPastEof,
            ));
        }
        Ok(self.file.read(buffer)?)
    }
}

impl Write for Filestream {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.file.write(bytes)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}
