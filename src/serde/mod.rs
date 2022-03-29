//! Serializer and deserializer for binary data.
mod deserializer;
mod error;
mod serializer;

use serde_core::{Serialize, de::DeserializeOwned};

use crate::{Endian, MemoryStream, BinaryWriter, BinaryReader};

pub use {error::Error, serializer::Serializer, deserializer::Deserializer};

/// Result type for serialization and deserialization.
pub type Result<T> = std::result::Result<T, Error>;

/// Serialize to an owned buffer.
pub fn to_vec<T>(value: &T, endian: Endian) -> Result<Vec<u8>> where
    T: ?Sized + Serialize {
    let mut stream = MemoryStream::new();
    let writer = BinaryWriter::new(&mut stream, endian);
    let mut serializer = Serializer { writer };
    value.serialize(&mut serializer)?;
    Ok(stream.into())
}

/// Deserialize from an owned buffer.
pub fn from_vec<T>(value: Vec<u8>, endian: Endian) -> Result<T> where
    T: DeserializeOwned {
    let mut stream: MemoryStream = value.into();
    let reader = BinaryReader::new(&mut stream, endian);
    let mut deserializer = Deserializer { reader };
    let value: T = serde_core::de::Deserialize::deserialize(&mut deserializer)?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use super::*;

    #[test]
    fn serde_unit() -> Result<()> {
        let val = ();
        let buffer = to_vec(&val, Default::default())?;
        let res: () = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_string() -> Result<()> {
        let val = String::from("foo");
        let buffer = to_vec(&val, Default::default())?;
        let res: String = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_bool_true() -> Result<()> {
        let val = true;
        let buffer = to_vec(&val, Default::default())?;
        let res: bool = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_bool_false() -> Result<()> {
        let val = false;
        let buffer = to_vec(&val, Default::default())?;
        let res: bool = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_u8() -> Result<()> {
        let val = 8u8;
        let buffer = to_vec(&val, Default::default())?;
        let res: u8 = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_u16() -> Result<()> {
        let val = 16u16;
        let buffer = to_vec(&val, Default::default())?;
        let res: u16 = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_u32() -> Result<()> {
        let val = 32u32;
        let buffer = to_vec(&val, Default::default())?;
        let res: u32 = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_u64() -> Result<()> {
        let val = 64u64;
        let buffer = to_vec(&val, Default::default())?;
        let res: u64 = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_usize() -> Result<()> {
        let val = usize::MAX;
        let buffer = to_vec(&val, Default::default())?;
        let res: usize = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_i8() -> Result<()> {
        let val = -8i8;
        let buffer = to_vec(&val, Default::default())?;
        let res: i8 = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_i16() -> Result<()> {
        let val = -16i16;
        let buffer = to_vec(&val, Default::default())?;
        let res: i16 = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_i32() -> Result<()> {
        let val = -32i32;
        let buffer = to_vec(&val, Default::default())?;
        let res: i32 = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_i64() -> Result<()> {
        let val = -64i64;
        let buffer = to_vec(&val, Default::default())?;
        let res: i64 = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_isize() -> Result<()> {
        let val = isize::MIN;
        let buffer = to_vec(&val, Default::default())?;
        let res: isize = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_vec() -> Result<()> {
        let val = vec![1u8, 2u8, 3u8];
        let buffer = to_vec(&val, Default::default())?;
        let res: Vec<u8> = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_tuple() -> Result<()> {
        let val = (1u8, String::from("foo"));
        let buffer = to_vec(&val, Default::default())?;
        let res: (u8, String) = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }
}
