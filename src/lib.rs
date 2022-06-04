//! Library for reading and writing binary data.
//!
//! If the `wasm32` feature is enabled then length
//! prefixed strings use a `u32` for the length so
//! that the encoded data is portable across platforms.
//!
//! Otherwise string length is encoded using `usize` which
//! may vary across platforms.
#![deny(missing_docs)]
use std::io::{Read, Write};

mod error;
mod stream;

pub use error::BinaryError;
pub use stream::file::{FileStream, OpenType};
pub use stream::memory::MemoryStream;
pub use stream::slice::SliceStream;

/// Result type for binary errors.
pub type Result<T> = std::result::Result<T, BinaryError>;

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
    /// Big endian.
    Big,
    /// Little endian.
    Little,
}

impl Default for Endian {
    fn default() -> Self {
        Self::Big
    }
}

/// Trait for streams that can seek.
pub trait SeekStream {
    /// Seek to a position.
    fn seek(&mut self, to: usize) -> Result<usize>;
    /// Get the current position.
    fn tell(&mut self) -> Result<usize>;
    /// Get the length of the stream.
    fn len(&self) -> Result<usize>;
}

/// Trait for a readable stream.
pub trait ReadStream: Read + SeekStream {}

/// Trait for a writable stream.
pub trait WriteStream: Write + SeekStream {}

/// Read from a stream.
pub struct BinaryReader<'a> {
    stream: &'a mut dyn ReadStream,
    endian: Endian,
}

impl<'a> BinaryReader<'a> {
    /// Create a binary reader with the given endianness.
    pub fn new(stream: &'a mut impl ReadStream, endian: Endian) -> Self {
        Self { stream, endian }
    }

    /// Seek to a position.
    pub fn seek(&mut self, to: usize) -> Result<usize> {
        Ok(self.stream.seek(to)?)
    }

    /// Get the current position.
    pub fn tell(&mut self) -> Result<usize> {
        Ok(self.stream.tell()?)
    }

    /// Read a length-prefixed `String` from the stream.
    pub fn read_string(&mut self) -> Result<String> {
        let chars = if cfg!(feature = "wasm32") {
            let str_len = self.read_u32()?;
            let mut chars: Vec<u8> = vec![0; str_len as usize];
            self.stream.read(&mut chars)?;
            chars
        } else {
            let str_len = self.read_usize()?;
            let mut chars: Vec<u8> = vec![0; str_len];
            self.stream.read(&mut chars)?;
            chars
        };
        Ok(String::from_utf8(chars)?)
    }

    /// Read a character from the stream.
    pub fn read_char(&mut self) -> Result<char> {
        Ok(std::char::from_u32(self.read_u32()?).ok_or_else(|| BinaryError::InvalidChar)?)
    }

    /// Read a `bool` from the stream.
    pub fn read_bool(&mut self) -> Result<bool> {
        let value = self.read_u8()?;
        Ok(value > 0)
    }

    /// Read a `f32` from the stream.
    pub fn read_f32(&mut self) -> Result<f32> {
        let mut buffer: [u8; 4] = [0; 4];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, f32);
    }

    /// Read a `f64` from the stream.
    pub fn read_f64(&mut self) -> Result<f64> {
        let mut buffer: [u8; 8] = [0; 8];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, f64);
    }

    /// Read an `isize` from the stream.
    #[cfg(target_arch = "wasm32")]
    pub fn read_isize(&mut self) -> Result<isize> {
        let mut buffer: [u8; 4] = [0; 4];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, isize);
    }

    /// Read an `isize` from the stream.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_isize(&mut self) -> Result<isize> {
        let mut buffer: [u8; 8] = [0; 8];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, isize);
    }

    /// Read a `usize` from the stream.
    #[cfg(target_arch = "wasm32")]
    pub fn read_usize(&mut self) -> Result<usize> {
        let mut buffer: [u8; 4] = [0; 4];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, usize);
    }

    /// Read a `usize` from the stream.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_usize(&mut self) -> Result<usize> {
        let mut buffer: [u8; 8] = [0; 8];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, usize);
    }

    /// Read a `u64` from the stream.
    pub fn read_u64(&mut self) -> Result<u64> {
        let mut buffer: [u8; 8] = [0; 8];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, u64);
    }

    /// Read an `i64` from the stream.
    pub fn read_i64(&mut self) -> Result<i64> {
        let mut buffer: [u8; 8] = [0; 8];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, i64);
    }

    /// Read a `u32` from the stream.
    pub fn read_u32(&mut self) -> Result<u32> {
        let mut buffer: [u8; 4] = [0; 4];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, u32);
    }

    /// Read an `i32` from the stream.
    pub fn read_i32(&mut self) -> Result<i32> {
        let mut buffer: [u8; 4] = [0; 4];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, i32);
    }

    /// Read a `u16` from the stream.
    pub fn read_u16(&mut self) -> Result<u16> {
        let mut buffer: [u8; 2] = [0; 2];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, u16);
    }

    /// Read an `i16` from the stream.
    pub fn read_i16(&mut self) -> Result<i16> {
        let mut buffer: [u8; 2] = [0; 2];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, i16);
    }

    /// Read a `u8` from the stream.
    pub fn read_u8(&mut self) -> Result<u8> {
        let mut buffer: [u8; 1] = [0; 1];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, u8);
    }

    /// Read an `i8` from the stream.
    pub fn read_i8(&mut self) -> Result<i8> {
        let mut buffer: [u8; 1] = [0; 1];
        self.stream.read(&mut buffer)?;
        decode!(self.endian, buffer, i8);
    }

    /// Read bytes from the stream into a buffer.
    pub fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>> {
        let mut buffer: Vec<u8> = vec![0; length];
        self.stream.read(&mut buffer)?;
        Ok(buffer)
    }
}

/// Write to a stream.
pub struct BinaryWriter<'a> {
    stream: &'a mut dyn WriteStream,
    endian: Endian,
}

impl<'a> BinaryWriter<'a> {
    /// Create a binary writer with the given endianness.
    pub fn new(stream: &'a mut impl WriteStream, endian: Endian) -> Self {
        Self { stream, endian }
    }

    /// Seek to a position.
    pub fn seek(&mut self, to: usize) -> Result<usize> {
        Ok(self.stream.seek(to)?)
    }

    /// Get the current position.
    pub fn tell(&mut self) -> Result<usize> {
        Ok(self.stream.tell()?)
    }

    /// Write a length-prefixed `String` to the stream.
    ///
    /// The length of the `String` is written as a `usize`
    /// unless the `wasm32` feature is enabled
    /// in which case the length is a `u32`.
    pub fn write_string<S: AsRef<str>>(&mut self, value: S) -> Result<usize> {
        let bytes = value.as_ref().as_bytes();
        if cfg!(feature = "wasm32") {
            self.write_u32(bytes.len() as u32)?;
        } else {
            self.write_usize(bytes.len())?;
        }
        Ok(self.stream.write(&bytes.to_vec())?)
    }

    /// Write a character to the stream.
    pub fn write_char(&mut self, v: char) -> Result<usize> {
        self.write_u32(v as u32)
    }

    /// Write a `bool` to the stream.
    pub fn write_bool(&mut self, value: bool) -> Result<usize> {
        let written = self.write_u8(if value { 1 } else { 0 })?;
        Ok(written)
    }

    /// Write a `f32` to the stream.
    pub fn write_f32(&mut self, value: f32) -> Result<usize> {
        encode!(self.endian, &value, self.stream);
    }

    /// Write a `f64` to the stream.
    pub fn write_f64(&mut self, value: f64) -> Result<usize> {
        encode!(self.endian, &value, self.stream);
    }

    /// Write an `isize` to the stream.
    pub fn write_isize(&mut self, value: isize) -> Result<usize> {
        encode!(self.endian, &value, self.stream);
    }

    /// Write a `usize` to the stream.
    pub fn write_usize(&mut self, value: usize) -> Result<usize> {
        encode!(self.endian, &value, self.stream);
    }

    /// Write a `u64` to the stream.
    pub fn write_u64(&mut self, value: u64) -> Result<usize> {
        encode!(self.endian, &value, self.stream);
    }

    /// Write an `i64` to the stream.
    pub fn write_i64(&mut self, value: i64) -> Result<usize> {
        encode!(self.endian, &value, self.stream);
    }

    /// Write a `u32` to the stream.
    pub fn write_u32(&mut self, value: u32) -> Result<usize> {
        encode!(self.endian, &value, self.stream);
    }

    /// Write an `i32` to the stream.
    pub fn write_i32(&mut self, value: i32) -> Result<usize> {
        encode!(self.endian, &value, self.stream);
    }

    /// Write a `u16` to the stream.
    pub fn write_u16(&mut self, value: u16) -> Result<usize> {
        encode!(self.endian, &value, self.stream);
    }

    /// Write an `i16` to the stream.
    pub fn write_i16(&mut self, value: i16) -> Result<usize> {
        encode!(self.endian, &value, self.stream);
    }

    /// Write a `u8` to the stream.
    pub fn write_u8(&mut self, value: u8) -> Result<usize> {
        encode!(self.endian, &value, self.stream);
    }

    /// Write an `i8` to the stream.
    pub fn write_i8(&mut self, value: i8) -> Result<usize> {
        encode!(self.endian, &value, self.stream);
    }

    /// Write a byte buffer to the stream.
    pub fn write_bytes<B: AsRef<[u8]>>(&mut self, data: B) -> Result<usize> {
        Ok(self.stream.write(data.as_ref())?)
    }
}

/// Trait for encoding to binary.
pub trait Encode {
    /// Encode self into the binary writer.
    fn encode(&self, writer: &mut BinaryWriter) -> Result<()>;
}

/// Trait for decoding from binary.
pub trait Decode {
    /// Decode from the binary reader into self.
    fn decode(&mut self, reader: &mut BinaryReader) -> Result<()>;
}
