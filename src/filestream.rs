//! Stream for operating on files.
use crate::{Stream, Result, BinaryError};
use std::fs;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Read, SeekFrom, Write};
use std::path::Path;

/// Indicates how the stream should open the underlying file.
pub enum OpenType {
    /// Open and create the file if it does not exist.
    OpenAndCreate,
    /// Open the file for reading.
    Open,
}

/// Stream that wraps a file.
pub struct FileStream {
    file: fs::File,
}

impl FileStream {
    /// Create a file stream.
    pub fn new<P: AsRef<Path>>(path: P, open_type: OpenType) -> Result<FileStream> {
        let file = match open_type {
            OpenType::OpenAndCreate => fs::File::create(path)?,
            OpenType::Open => fs::File::open(path)?,
        };
        Ok(FileStream { file })
    }
}

impl Stream for FileStream {
    fn seek(&mut self, to: usize) -> Result<usize> {
        Ok(self.file.seek(SeekFrom::Start(to as u64))? as usize)
    }

    fn tell(&mut self) -> Result<usize> {
        Ok(self.file.seek(SeekFrom::Current(0))? as usize)
    }
}

impl Read for FileStream {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        if self.tell().unwrap() + buffer.len() > self.file.metadata()?.len() as usize {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                BinaryError::ReadPastEof,
            ));
        }
        Ok(self.file.read(buffer)?)
    }
}

impl Write for FileStream {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.file.write(bytes)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}
