extern crate bincode;

use bincode::{deserialize, deserialize_from, serialize};
use std::fs;
use std::io::prelude::*;
use std::io::SeekFrom;

pub enum OpenType {
    OpenAndCreate,
    Open,
}

pub struct BinaryReader {
    file: fs::File,
}

#[derive(Debug)]
pub enum BinaryError {
    FSError(std::io::Error),
    BinCodeErr(Box<bincode::ErrorKind>),
}

impl BinaryReader {
    pub fn new(filepath: &str, open_type: OpenType) -> Result<BinaryReader, BinaryError> {
        let file;

        match open_type {
            OpenType::OpenAndCreate => file = fs::File::create(filepath),
            OpenType::Open => file = fs::File::open(filepath),
        }

        match file {
            Ok(f) => Ok(BinaryReader { file: f }),
            Err(e) => Err(BinaryError::FSError(e)),
        }
    }

    pub fn seek_to(&mut self, position: u64) -> Result<u64, BinaryError> {
        let result = self.file.seek(SeekFrom::Start(position));

        match result {
            Ok(r) => Ok(r),
            Err(e) => Err(BinaryError::FSError(e)),
        }
    }

    pub fn get_cur_pos(&mut self) -> Result<u64, BinaryError> {
        let result = self.file.seek(SeekFrom::Current(0));

        match result {
            Ok(r) => Ok(r),
            Err(e) => Err(BinaryError::FSError(e)),
        }
    }

    pub fn read_string(&mut self) -> Result<String, BinaryError> {
        let value = deserialize_from(&self.file);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_f32(&mut self) -> Result<f32, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 4];

        let read = self.file.read(&mut buffer);

        match read {
            Err(e) => return Err(BinaryError::FSError(e)),
            _ => {}
        };

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_f64(&mut self) -> Result<f64, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 8];

        let read = self.file.read(&mut buffer);

        match read {
            Err(e) => return Err(BinaryError::FSError(e)),
            _ => {}
        };

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_isize(&mut self) -> Result<isize, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 8];

        let read = self.file.read(&mut buffer);

        match read {
            Err(e) => return Err(BinaryError::FSError(e)),
            _ => {}
        };

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_usize(&mut self) -> Result<usize, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 8];

        let read = self.file.read(&mut buffer);

        match read {
            Err(e) => return Err(BinaryError::FSError(e)),
            _ => {}
        };

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_u64(&mut self) -> Result<u64, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 8];

        let read = self.file.read(&mut buffer);

        match read {
            Err(e) => return Err(BinaryError::FSError(e)),
            _ => {}
        };

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_i64(&mut self) -> Result<i64, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 8];

        let read = self.file.read(&mut buffer);

        match read {
            Err(e) => return Err(BinaryError::FSError(e)),
            _ => {}
        };

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_u32(&mut self) -> Result<u32, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 4];

        let read = self.file.read(&mut buffer);

        match read {
            Err(e) => return Err(BinaryError::FSError(e)),
            _ => {}
        };

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_i32(&mut self) -> Result<i32, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 4];

        let read = self.file.read(&mut buffer);

        match read {
            Err(e) => return Err(BinaryError::FSError(e)),
            _ => {}
        };

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_u16(&mut self) -> Result<u16, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 2];

        let read = self.file.read(&mut buffer);

        match read {
            Err(e) => return Err(BinaryError::FSError(e)),
            _ => {}
        };

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_i16(&mut self) -> Result<i16, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 2];

        let read = self.file.read(&mut buffer);

        match read {
            Err(e) => return Err(BinaryError::FSError(e)),
            _ => {}
        };

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_u8(&mut self) -> Result<u8, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 1];

        let read = self.file.read(&mut buffer);

        match read {
            Err(e) => return Err(BinaryError::FSError(e)),
            _ => {}
        };

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_i8(&mut self) -> Result<i8, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; 1];

        let read = self.file.read(&mut buffer);

        match read {
            Err(e) => return Err(BinaryError::FSError(e)),
            _ => {}
        };

        let value = deserialize(&buffer);

        match value {
            Ok(v) => Ok(v),
            Err(e) => Err(BinaryError::BinCodeErr(e)),
        }
    }

    pub fn read_bytes(&mut self, length: u64) -> Result<Vec<u8>, BinaryError> {
        let mut buffer: Vec<u8> = vec![0; length as usize];
        let bytes = self.file.read(&mut buffer);

        match bytes {
            Ok(_) => Ok(buffer),
            Err(e) => Err(BinaryError::FSError(e)),
        }
    }
}

pub struct BinaryWriter {
    file: fs::File,
}

impl BinaryWriter {
    pub fn new(filepath: &str, open_type: OpenType) -> Result<BinaryWriter, BinaryError> {
        let file;

        match open_type {
            OpenType::OpenAndCreate => file = fs::File::create(filepath),
            OpenType::Open => file = fs::File::open(filepath),
        }

        match file {
            Ok(f) => Ok(BinaryWriter { file: f }),
            Err(e) => Err(BinaryError::FSError(e)),
        }
    }

    pub fn seek_to(&mut self, position: u64) -> Result<u64, BinaryError> {
        let result = self.file.seek(SeekFrom::Start(position));

        match result {
            Ok(seek) => Ok(seek),
            Err(e) => Err(BinaryError::FSError(e)),
        }
    }

    pub fn get_cur_pos(&mut self) -> Result<u64, BinaryError> {
        let result = self.file.seek(SeekFrom::Current(0));

        match result {
            Ok(pos) => Ok(pos),
            Err(e) => Err(BinaryError::FSError(e)),
        }
    }

    pub fn write_string(&mut self, value: String) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.file.write(&data);

        match result {
            Err(e) => Some(BinaryError::FSError(e)),
            _ => None,
        }
    }

    pub fn write_f32(&mut self, value: f32) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.file.write(&data);

        match result {
            Err(e) => Some(BinaryError::FSError(e)),
            _ => None,
        }
    }

    pub fn write_f64(&mut self, value: f64) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.file.write(&data);

        match result {
            Err(e) => Some(BinaryError::FSError(e)),
            _ => None,
        }
    }

    pub fn write_isize(&mut self, value: isize) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.file.write(&data);

        match result {
            Err(e) => Some(BinaryError::FSError(e)),
            _ => None,
        }
    }

    pub fn write_usize(&mut self, value: usize) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.file.write(&data);

        match result {
            Err(e) => Some(BinaryError::FSError(e)),
            _ => None,
        }
    }

    pub fn write_u64(&mut self, value: u64) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.file.write(&data);

        match result {
            Err(e) => Some(BinaryError::FSError(e)),
            _ => None,
        }
    }

    pub fn write_i64(&mut self, value: i64) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.file.write(&data);

        match result {
            Err(e) => Some(BinaryError::FSError(e)),
            _ => None,
        }
    }

    pub fn write_u32(&mut self, value: u32) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.file.write(&data);

        match result {
            Err(e) => Some(BinaryError::FSError(e)),
            _ => None,
        }
    }

    pub fn write_i32(&mut self, value: i32) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.file.write(&data);

        match result {
            Err(e) => Some(BinaryError::FSError(e)),
            _ => None,
        }
    }

    pub fn write_u16(&mut self, value: u16) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.file.write(&data);

        match result {
            Err(e) => Some(BinaryError::FSError(e)),
            _ => None,
        }
    }

    pub fn write_i16(&mut self, value: i16) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.file.write(&data);

        match result {
            Err(e) => Some(BinaryError::FSError(e)),
            _ => None,
        }
    }

    pub fn write_u8(&mut self, value: u8) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.file.write(&data);

        match result {
            Err(e) => Some(BinaryError::FSError(e)),
            _ => None,
        }
    }

    pub fn write_i8(&mut self, value: i8) -> Option<BinaryError> {
        let data = serialize(&value);

        let data = match data {
            Ok(d) => d,
            Err(e) => return Some(BinaryError::BinCodeErr(e)),
        };
        let result = self.file.write(&data);

        match result {
            Err(e) => Some(BinaryError::FSError(e)),
            _ => None,
        }
    }

    pub fn write_bytes(&mut self, data: Vec<u8>) -> Option<BinaryError> {
        let result = self.file.write(&data);

        match result {
            Err(e) => Some(BinaryError::FSError(e)),
            _ => None,
        }
    }
}
