//! Write a `Serialize` implementation to a binary writer.
use serde::ser::{self, Serialize};
use crate::BinaryWriter;
use super::{Result, Error};

#[doc(hidden)]
pub struct SerializeArray<'a, 'b> {
    ser: &'a mut Serializer<'b>,
}

impl<'a, 'b> ser::SerializeSeq for SerializeArray<'a, 'b> {
    type Ok = usize;
    type Error = Error;

    fn serialize_element<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(0)
    }
}

impl<'a, 'b> ser::SerializeTuple for SerializeArray<'a, 'b> {
    type Ok = usize;
    type Error = Error;

    fn serialize_element<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, 'b> ser::SerializeTupleStruct for SerializeArray<'a, 'b> {
    type Ok = usize;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeSeq::end(self)
    }
}

#[doc(hidden)]
pub struct SerializeObject<'a, 'b> {
    ser: &'a mut Serializer<'b>,
}

impl<'a, 'b> ser::SerializeStruct for SerializeObject<'a, 'b> {
    type Ok = usize;
    type Error = Error;
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        self.ser.writer.write_string(key)?;
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(0)
    }

    fn skip_field(&mut self, _key: &'static str) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'b> ser::SerializeMap for SerializeObject<'a, 'b> {
    type Ok = usize;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn serialize_value<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(0)
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<()>
    where
        K: Serialize,
        V: Serialize,
    {
        self.serialize_key(key)?;
        self.serialize_value(value)?;
        Ok(())
    }
}

impl<'a, 'b> ser::SerializeTupleVariant for SerializeArray<'a, 'b> {
    type Ok = usize;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(0)
    }
}

impl<'a, 'b> ser::SerializeStructVariant for SerializeObject<'a, 'b> {
    type Ok = usize;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        self.ser.writer.write_string(key)?;
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(0)
    }

    fn skip_field(&mut self, _key: &'static str) -> Result<()> {
        Ok(())
    }
}

/// Serializer for binary data.
pub struct Serializer<'a> {
    /// The binary writer to write to.
    pub writer: BinaryWriter<'a>,
}

impl<'a, 'b> ser::Serializer for &'a mut Serializer<'b> {
    type Ok = usize;
    type Error = Error;

    type SerializeSeq = SerializeArray<'a, 'b>;
    type SerializeTuple = SerializeArray<'a, 'b>;
    type SerializeTupleStruct = SerializeArray<'a, 'b>;
    type SerializeTupleVariant = SerializeArray<'a, 'b>;
    type SerializeMap = SerializeObject<'a, 'b>;
    type SerializeStruct = SerializeObject<'a, 'b>;
    type SerializeStructVariant = SerializeObject<'a, 'b>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        Ok(self.writer.write_bool(v)?)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        Ok(self.writer.write_i8(v)?)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        Ok(self.writer.write_i16(v)?)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        Ok(self.writer.write_i32(v)?)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        Ok(self.writer.write_i64(v)?)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        Ok(self.writer.write_u8(v)?)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        Ok(self.writer.write_u16(v)?)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        Ok(self.writer.write_u32(v)?)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        Ok(self.writer.write_u64(v)?)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        Ok(self.writer.write_f32(v)?)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        Ok(self.writer.write_f64(v)?)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        // A char encoded as UTF-8 takes 4 bytes at most.
        let mut buf = [0; 4];
        self.serialize_str(v.encode_utf8(&mut buf))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        Ok(self.writer.write_string(v)?)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        Ok(self.writer.write_bytes(v)?)
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(self.writer.write_u8(0)?)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        self.writer.write_u8(1)?;
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(self.writer.write_u8(0)?)
    }

    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.writer.write_string(variant)?;
        self.serialize_unit()
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        self.writer.write_string(variant)?;
        value.serialize(self)
    }

    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeSeq> {
        self.writer.write_u32(len.map(|l| l as u32).unwrap_or(0))?;
        Ok(SerializeArray {
            ser: self,
        })
    }

    fn serialize_tuple(
        self,
        len: usize,
    ) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        //println!("Serialize a tuple struct!");
        self.writer.write_string(name)?;
        self.serialize_seq(Some(len))
    }

    fn serialize_map(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeMap> {
        self.writer.write_u32(len.map(|l| l as u32).unwrap_or(0))?;
        Ok(SerializeObject {
            ser: self,
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.writer.write_string(variant)?;
        self.serialize_seq(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.writer.write_string(variant)?;
        self.serialize_struct(variant, len)
    }
}
