//! Stream for operating on files.
use crate::{BinaryError, ReadStream, Result, SeekStream, WriteStream};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Read, SeekFrom, Write};
use std::path::Path;

/// Stream that wraps a file.
pub struct FileStream(File);

impl FileStream {
    /// Create a file stream.
    pub fn new(file: File) -> Self {
        Self(file)
    }

    /// Create a file stream in write-only mode.
    ///
    /// If the file exists it is truncated, if it does not 
    /// exist it will be created.
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self(File::create(path.as_ref())?))
    }

    /// Attempts to open a file stream in read-only mode.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self(File::open(path.as_ref())?))
    }

    /// Attempts to open a file stream with read and write modes enabled.
    pub fn write<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self(OpenOptions::new().read(true).write(true).open(path)?))
    }
}

impl SeekStream for FileStream {
    fn seek(&mut self, to: usize) -> Result<usize> {
        Ok(self.0.seek(SeekFrom::Start(to as u64))? as usize)
    }

    fn tell(&mut self) -> Result<usize> {
        Ok(self.0.seek(SeekFrom::Current(0))? as usize)
    }

    fn len(&self) -> Result<usize> {
        Ok(self.0.metadata()?.len().try_into()?)
    }
}

impl Read for FileStream {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        if self.tell().unwrap() + buffer.len() > self.0.metadata()?.len() as usize {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                BinaryError::ReadPastEof,
            ));
        }
        Ok(self.0.read(buffer)?)
    }
}

impl Write for FileStream {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.write(bytes)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

impl ReadStream for FileStream {}
impl WriteStream for FileStream {}
