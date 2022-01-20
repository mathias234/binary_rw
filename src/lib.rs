use std::io::{Read, Write};
use std::string::FromUtf8Error;
use thiserror::Error;

pub mod filestream;
pub mod memorystream;

macro_rules! encode {
    ($endian:expr, $value:expr, $stream:expr) => {
        let data = match $endian {
            Endian::Little => $value.to_le_bytes(),
            Endian::Big => $value.to_be_bytes(),
        };
        return Ok($stream.write(&data)?);
    };
}

macro_rules! decode {
    ($endian:expr, $value:expr, $kind:ty) => {
        let data = match $endian {
            Endian::Little => <$kind>::from_le_bytes($value),
            Endian::Big => <$kind>::from_be_bytes($value),
        };
        return Ok(data);
    };
}

/// Variants to describe endianness.
pub enum Endian {
    Big,
    Little,
}

impl Default for Endian {
    fn default() -> Self {
        Self::Big
    }
}

#[derive(Debug, Error)]
pub enum BinaryError {
    #[error(transparent)]
    StreamError(StreamError),
    #[error(transparent)]
    Utf8Error(FromUtf8Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl From<FromUtf8Error> for BinaryError {
    fn from(error: FromUtf8Error) -> BinaryError {
        BinaryError::Utf8Error(error)
    }
}

impl From<StreamError> for BinaryError {
    fn from(error: StreamError) -> BinaryError {
        BinaryError::StreamError(error)
    }
}

#[derive(Debug, Error)]
pub enum StreamError {
    #[error("attempt to read past EOF")]
    ReadPastEof,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub trait Stream: Read + Write {
    fn seek(&mut self, to: usize) -> Result<usize, StreamError>;
    fn tell(&mut self) -> Result<usize, StreamError>;
}

pub struct BinaryReader<'a> {
    stream: &'a mut dyn Stream,
    endian: Endian,
}

impl<'a> BinaryReader<'a> {
    pub fn new(stream: &'a mut impl Stream) -> BinaryReader {
        Self::new_endian(stream, Default::default())
    }

    pub fn new_endian(stream: &'a mut impl Stream, endian: Endian) -> BinaryReader {
        BinaryReader { stream, endian }
    }

    pub fn seek_to(&mut self, to: usize) -> Result<usize, BinaryError> {
        Ok(self.stream.seek(to)?)
    }

    pub fn get_cur_pos(&mut self) -> Result<usize, BinaryError> {
        Ok(self.stream.tell()?)
    }

    pub fn read_string(&mut self) -> Result<String, BinaryError> {
        let str_len = self.read_usize()?;
        let mut chars: Vec<u8> = vec![0; str_len];
        self.stream.read(&mut chars)?;
        Ok(String::from_utf8(chars)?)
    }

    pub fn read_bool(&mut self) -> Result<bool, BinaryError> {
        let value = self.read_u8()?;
        Ok(value > 0)
    }

    pub fn read_f32(&mut self) -> Result<f32, BinaryError> {
        let mut buffer: [u8; 4] = [0; 4];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, f32);
    }

    pub fn read_f64(&mut self) -> Result<f64, BinaryError> {
        let mut buffer: [u8; 8] = [0; 8];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, f64);
    }

    pub fn read_isize(&mut self) -> Result<isize, BinaryError> {
        let mut buffer: [u8; 8] = [0; 8];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, isize);
    }

    pub fn read_usize(&mut self) -> Result<usize, BinaryError> {
        let mut buffer: [u8; 8] = [0; 8];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, usize);
    }

    pub fn read_u64(&mut self) -> Result<u64, BinaryError> {
        let mut buffer: [u8; 8] = [0; 8];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, u64);
    }

    pub fn read_i64(&mut self) -> Result<i64, BinaryError> {
        let mut buffer: [u8; 8] = [0; 8];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, i64);
    }

    pub fn read_u32(&mut self) -> Result<u32, BinaryError> {
        let mut buffer: [u8; 4] = [0; 4];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, u32);
    }

    pub fn read_i32(&mut self) -> Result<i32, BinaryError> {
        let mut buffer: [u8; 4] = [0; 4];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, i32);
    }

    pub fn read_u16(&mut self) -> Result<u16, BinaryError> {
        let mut buffer: [u8; 2] = [0; 2];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, u16);
    }

    pub fn read_i16(&mut self) -> Result<i16, BinaryError> {
        let mut buffer: [u8; 2] = [0; 2];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, i16);
    }

    pub fn read_u8(&mut self) -> Result<u8, BinaryError> {
        let mut buffer: [u8; 1] = [0; 1];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, u8);
    }

    pub fn read_i8(&mut self) -> Result<i8, BinaryError> {
        let mut buffer: [u8; 1] = [0; 1];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, i8);
    }

    pub fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; length];
        self.stream.read(&mut buffer)?;
        Ok(buffer)
    }
}

pub struct BinaryWriter<'a> {
    stream: &'a mut dyn Stream,
    endian: Endian,
}

impl<'a> BinaryWriter<'a> {
    pub fn new(stream: &'a mut impl Stream) -> BinaryWriter {
        Self::new_endian(stream, Default::default())
    }

    pub fn new_endian(stream: &'a mut impl Stream, endian: Endian) -> BinaryWriter {
        BinaryWriter { stream, endian }
    }

    pub fn seek_to(&mut self, to: usize) -> Result<usize, BinaryError> {
        Ok(self.stream.seek(to)?)
    }

    pub fn get_cur_pos(&mut self) -> Result<usize, BinaryError> {
        Ok(self.stream.tell()?)
    }

    pub fn write_string<S: AsRef<str>>(&mut self, value: S) -> Result<usize, BinaryError> {
        let bytes = value.as_ref().as_bytes();
        self.write_usize(bytes.len())?;
        Ok(self.stream.write(&bytes.to_vec())?)
    }

    pub fn write_bool(&mut self, value: bool) -> Result<usize, BinaryError> {
        let written = self.write_u8(if value { 1 } else { 0 })?;
        Ok(written)
    }

    pub fn write_f32(&mut self, value: f32) -> Result<usize, BinaryError> {
        encode!(self.endian, &value, self.stream);
    }

    pub fn write_f64(&mut self, value: f64) -> Result<usize, BinaryError> {
        encode!(self.endian, &value, self.stream);
    }

    pub fn write_isize(&mut self, value: isize) -> Result<usize, BinaryError> {
        encode!(self.endian, &value, self.stream);
    }

    pub fn write_usize(&mut self, value: usize) -> Result<usize, BinaryError> {
        encode!(self.endian, &value, self.stream);
    }

    pub fn write_u64(&mut self, value: u64) -> Result<usize, BinaryError> {
        encode!(self.endian, &value, self.stream);
    }

    pub fn write_i64(&mut self, value: i64) -> Result<usize, BinaryError> {
        encode!(self.endian, &value, self.stream);
    }

    pub fn write_u32(&mut self, value: u32) -> Result<usize, BinaryError> {
        encode!(self.endian, &value, self.stream);
    }

    pub fn write_i32(&mut self, value: i32) -> Result<usize, BinaryError> {
        encode!(self.endian, &value, self.stream);
    }

    pub fn write_u16(&mut self, value: u16) -> Result<usize, BinaryError> {
        encode!(self.endian, &value, self.stream);
    }

    pub fn write_i16(&mut self, value: i16) -> Result<usize, BinaryError> {
        encode!(self.endian, &value, self.stream);
    }

    pub fn write_u8(&mut self, value: u8) -> Result<usize, BinaryError> {
        encode!(self.endian, &value, self.stream);
    }

    pub fn write_i8(&mut self, value: i8) -> Result<usize, BinaryError> {
        encode!(self.endian, &value, self.stream);
    }

    pub fn write_bytes(&mut self, data: Vec<u8>) -> Result<usize, BinaryError> {
        Ok(self.stream.write(&data)?)
    }
}
