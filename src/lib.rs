extern crate bincode;

use std::string::FromUtf8Error;
use thiserror::Error;

use bincode::{deserialize, serialize};

pub mod filestream;
pub mod memorystream;

pub struct BinaryReader<'a> {
    stream: &'a mut dyn Stream,
}

#[derive(Debug, Error)]
pub enum BinaryError {
    #[error(transparent)]
    StreamError(StreamError),
    #[error(transparent)]
    BinCodeErr(Box<bincode::ErrorKind>),
    #[error(transparent)]
    Utf8Error(FromUtf8Error),
}

impl From<FromUtf8Error> for BinaryError {
    fn from(error: FromUtf8Error) -> BinaryError {
        BinaryError::Utf8Error(error)
    }
}

impl From<Box<bincode::ErrorKind>> for BinaryError {
    fn from(error: Box<bincode::ErrorKind>) -> BinaryError {
        BinaryError::BinCodeErr(error)
    }
}

impl From<StreamError> for BinaryError {
    fn from(error: StreamError) -> BinaryError {
        BinaryError::StreamError(error)
    }
}

#[derive(Debug, Error)]
pub enum StreamError {
    #[error("failed to open stream")]
    OpenError,
    #[error("failed to write to stream")]
    WriteError,
    #[error("failed to read from stream")]
    ReadError,
    #[error("failed to seek in stream")]
    SeekError,
    #[error("failed to tell in stream")]
    TellError,
}

pub trait Stream {
    fn write(&mut self, bytes: &Vec<u8>) -> Result<usize, StreamError>;
    fn read(&mut self, buffer: &mut Vec<u8>) -> Result<usize, StreamError>;
    fn seek(&mut self, to: usize) -> Result<usize, StreamError>;
    fn tell(&mut self) -> Result<usize, StreamError>;
}

impl<'a> BinaryReader<'a> {
    pub fn new(stream: &'a mut impl Stream) -> BinaryReader {
        BinaryReader { stream }
    }

    pub fn seek_to(&mut self, to: usize) -> Result<usize, BinaryError> {
        let result = self.stream.seek(to);

        match result {
            Ok(r) => Ok(r),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }

    pub fn get_cur_pos(&mut self) -> Result<usize, BinaryError> {
        let result = self.stream.tell();

        match result {
            Ok(r) => Ok(r),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }

    pub fn read_string(&mut self) -> Result<String, BinaryError> {
        let str_len = self.read_usize()?;

        let mut chars: Vec<u8> = vec![0; str_len];
        self.stream.read(&mut chars)?;

        let string = String::from_utf8(chars)?;
        Ok(string)
    }

    pub fn read_f32(&mut self) -> Result<f32, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 4];

        self.stream.read(&mut buffer)?;

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_f64(&mut self) -> Result<f64, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 8];

        self.stream.read(&mut buffer)?;

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_isize(&mut self) -> Result<isize, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 8];

        self.stream.read(&mut buffer)?;

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_usize(&mut self) -> Result<usize, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 8];

        self.stream.read(&mut buffer)?;

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_u64(&mut self) -> Result<u64, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 8];

        self.stream.read(&mut buffer)?;

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_i64(&mut self) -> Result<i64, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 8];

        self.stream.read(&mut buffer)?;

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_u32(&mut self) -> Result<u32, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 4];

        self.stream.read(&mut buffer)?;

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_i32(&mut self) -> Result<i32, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 4];

        self.stream.read(&mut buffer)?;

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_u16(&mut self) -> Result<u16, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 2];

        self.stream.read(&mut buffer)?;

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_i16(&mut self) -> Result<i16, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 2];

        self.stream.read(&mut buffer)?;

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_u8(&mut self) -> Result<u8, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 1];

        self.stream.read(&mut buffer)?;

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_i8(&mut self) -> Result<i8, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 1];

        self.stream.read(&mut buffer)?;

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; length];
        let bytes = self.stream.read(&mut buffer);

        match bytes {
            Ok(_) => Ok(buffer),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }
}

pub struct BinaryWriter<'a> {
    stream: &'a mut dyn Stream,
}

impl<'a> BinaryWriter<'a> {
    pub fn new(stream: &'a mut impl Stream) -> BinaryWriter {
        BinaryWriter { stream }
    }

    pub fn seek_to(&mut self, to: usize) -> Result<usize, BinaryError> {
        let result = self.stream.seek(to);

        match result {
            Ok(r) => Ok(r),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }

    pub fn get_cur_pos(&mut self) -> Result<usize, BinaryError> {
        let result = self.stream.tell();

        match result {
            Ok(pos) => Ok(pos),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }

    pub fn write_string(&mut self, value: String) -> Result<usize, BinaryError> {
        let bytes = value.as_bytes();

        self.write_usize(bytes.len())?;

        let result = self.stream.write(&bytes.to_vec());

        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }

    pub fn write_f32(&mut self, value: f32) -> Result<usize, BinaryError> {
        let data = serialize(&value)?;

        let result = self.stream.write(&data);

        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }

    pub fn write_f64(&mut self, value: f64) -> Result<usize, BinaryError> {
        let data = serialize(&value)?;

        let result = self.stream.write(&data);

        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }

    pub fn write_isize(&mut self, value: isize) -> Result<usize, BinaryError> {
        let data = serialize(&value)?;

        let result = self.stream.write(&data);

        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }

    pub fn write_usize(&mut self, value: usize) -> Result<usize, BinaryError> {
        let data = serialize(&value)?;

        let result = self.stream.write(&data);

        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }

    pub fn write_u64(&mut self, value: u64) -> Result<usize, BinaryError> {
        let data = serialize(&value)?;

        let result = self.stream.write(&data);

        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }

    pub fn write_i64(&mut self, value: i64) -> Result<usize, BinaryError> {
        let data = serialize(&value)?;

        let result = self.stream.write(&data);

        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }

    pub fn write_u32(&mut self, value: u32) -> Result<usize, BinaryError> {
        let data = serialize(&value)?;

        let result = self.stream.write(&data);

        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }

    pub fn write_i32(&mut self, value: i32) -> Result<usize, BinaryError> {
        let data = serialize(&value)?;

        let result = self.stream.write(&data);

        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }

    pub fn write_u16(&mut self, value: u16) -> Result<usize, BinaryError> {
        let data = serialize(&value)?;

        let result = self.stream.write(&data);

        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }

    pub fn write_i16(&mut self, value: i16) -> Result<usize, BinaryError> {
        let data = serialize(&value)?;

        let result = self.stream.write(&data);

        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }

    pub fn write_u8(&mut self, value: u8) -> Result<usize, BinaryError> {
        let data = serialize(&value)?;

        let result = self.stream.write(&data);

        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }

    pub fn write_i8(&mut self, value: i8) -> Result<usize, BinaryError> {
        let data = serialize(&value)?;

        let result = self.stream.write(&data);

        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }

    pub fn write_bytes(&mut self, data: Vec<u8>) -> Result<usize, BinaryError> {
        let result = self.stream.write(&data);

        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }
}
