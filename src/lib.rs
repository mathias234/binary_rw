extern crate bincode;

use std::string::FromUtf8Error;

use bincode::{deserialize, serialize};

pub mod filestream;

pub struct BinaryReader<'a> {
    stream: &'a mut dyn Stream,
}

#[derive(Debug)]
pub enum BinaryError {
    StreamError(StreamError),
    BinCodeErr(Box<bincode::ErrorKind>),
    Utf8Error(FromUtf8Error),
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

#[derive(Debug)]
pub enum StreamError {
    OpenError,
    WriteError,
    ReadError,
    SeekError,
    TellError,
}

pub trait Stream {
    fn write(&mut self, bytes: &Vec<u8>) -> Result<u64, StreamError>;
    fn read(&mut self, buffer: &mut Vec<u8>) -> Result<u64, StreamError>;
    fn seek(&mut self, from: u64) -> Result<u64, StreamError>;
    fn tell(&mut self) -> Result<u64, StreamError>;
}

impl<'a> BinaryReader<'a> {
    pub fn new(stream: &'a mut impl Stream) -> BinaryReader {
        BinaryReader { stream }
    }

    pub fn seek_to(&mut self, to: u64) -> Result<u64, BinaryError> {
        let result = self.stream.seek(to);

        match result {
            Ok(r) => Ok(r),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }

    pub fn get_cur_pos(&mut self) -> Result<u64, BinaryError> {
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

    pub fn read_bytes(&mut self, length: u64) -> Result<Vec<u8>, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; length as usize];
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

    pub fn seek_to(&mut self, to: u64) -> Result<u64, BinaryError> {
        let result = self.stream.seek(to);

        match result {
            Ok(r) => Ok(r),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }

    pub fn get_cur_pos(&mut self) -> Result<u64, BinaryError> {
        let result = self.stream.tell();

        match result {
            Ok(pos) => Ok(pos),
            Err(e) => Err(BinaryError::StreamError(e)),
        }
    }

    pub fn write_string(&mut self, value: String) -> Option<BinaryError> {
        let bytes = value.as_bytes();

        self.write_usize(bytes.len());
        let result = self.stream.write(&bytes.to_vec());

        match result {
            Err(e) => Some(BinaryError::StreamError(e)),
            _ => None,
        }
    }

    pub fn write_f32(&mut self, value: f32) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.stream.write(&data);

        match result {
            Err(e) => Some(BinaryError::StreamError(e)),
            _ => None,
        }
    }

    pub fn write_f64(&mut self, value: f64) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.stream.write(&data);

        match result {
            Err(e) => Some(BinaryError::StreamError(e)),
            _ => None,
        }
    }

    pub fn write_isize(&mut self, value: isize) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.stream.write(&data);

        match result {
            Err(e) => Some(BinaryError::StreamError(e)),
            _ => None,
        }
    }

    pub fn write_usize(&mut self, value: usize) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.stream.write(&data);

        match result {
            Err(e) => Some(BinaryError::StreamError(e)),
            _ => None,
        }
    }

    pub fn write_u64(&mut self, value: u64) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.stream.write(&data);

        match result {
            Err(e) => Some(BinaryError::StreamError(e)),
            _ => None,
        }
    }

    pub fn write_i64(&mut self, value: i64) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.stream.write(&data);

        match result {
            Err(e) => Some(BinaryError::StreamError(e)),
            _ => None,
        }
    }

    pub fn write_u32(&mut self, value: u32) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.stream.write(&data);

        match result {
            Err(e) => Some(BinaryError::StreamError(e)),
            _ => None,
        }
    }

    pub fn write_i32(&mut self, value: i32) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.stream.write(&data);

        match result {
            Err(e) => Some(BinaryError::StreamError(e)),
            _ => None,
        }
    }

    pub fn write_u16(&mut self, value: u16) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.stream.write(&data);

        match result {
            Err(e) => Some(BinaryError::StreamError(e)),
            _ => None,
        }
    }

    pub fn write_i16(&mut self, value: i16) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.stream.write(&data);

        match result {
            Err(e) => Some(BinaryError::StreamError(e)),
            _ => None,
        }
    }

    pub fn write_u8(&mut self, value: u8) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.stream.write(&data);

        match result {
            Err(e) => Some(BinaryError::StreamError(e)),
            _ => None,
        }
    }

    pub fn write_i8(&mut self, value: i8) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.stream.write(&data);

        match result {
            Err(e) => Some(BinaryError::StreamError(e)),
            _ => None,
        }
    }

    pub fn write_bytes(&mut self, data: Vec<u8>) -> Option<BinaryError> {
        let result = self.stream.write(&data);

        match result {
            Err(e) => Some(BinaryError::StreamError(e)),
            _ => None,
        }
    }
}
