use super::{Error, Result};
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant, Serializer as SerializerTrait,
};
use serde::Serialize;

/// Struct responsible for serializing to custom data format.
#[derive(Clone, Debug, Default)]
pub struct Serializer(String);

impl Serializer {
    /// Creates new instance of `Serializer`
    /// and initializes it with empty string buffer with capacity of `capacity`.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(String::with_capacity(capacity))
    }

    fn ensure_new_line(&mut self) {
        if !self.0.is_empty() && !self.0.ends_with('\n') {
            if self.0.ends_with(' ') {
                self.0.pop();
            }
            self.0.push('\n');
        }
    }

    fn ensure_white_space(&mut self) {
        if !self.0.ends_with(' ') && !self.0.ends_with('\n') && !self.0.is_empty() {
            self.0.push(' ');
        }
    }

    fn add_to_buffer<T: ToString>(&mut self, value: T) {
        self.0.push_str(&value.to_string());
    }
}

impl SerializerTrait for &mut Serializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, value: bool) -> Result<()> {
        self.add_to_buffer(value);
        Ok(())
    }

    fn serialize_i8(self, value: i8) -> Result<()> {
        self.serialize_i64(value.into())
    }

    fn serialize_i16(self, value: i16) -> Result<()> {
        self.serialize_i64(value.into())
    }

    fn serialize_i32(self, value: i32) -> Result<()> {
        self.serialize_i64(value.into())
    }

    fn serialize_i64(self, value: i64) -> Result<()> {
        self.add_to_buffer(value);
        Ok(())
    }

    fn serialize_i128(self, value: i128) -> Result<()> {
        self.add_to_buffer(value);
        Ok(())
    }

    fn serialize_u8(self, value: u8) -> Result<()> {
        self.serialize_u64(value.into())
    }

    fn serialize_u16(self, value: u16) -> Result<()> {
        self.serialize_u64(value.into())
    }

    fn serialize_u32(self, value: u32) -> Result<()> {
        self.serialize_u64(value.into())
    }

    fn serialize_u64(self, value: u64) -> Result<()> {
        self.add_to_buffer(value);
        Ok(())
    }

    fn serialize_u128(self, value: u128) -> Result<()> {
        self.add_to_buffer(value);
        Ok(())
    }

    fn serialize_f32(self, value: f32) -> Result<()> {
        self.serialize_f64(value.into())
    }

    fn serialize_f64(self, value: f64) -> Result<()> {
        self.add_to_buffer(value);
        Ok(())
    }

    fn serialize_char(self, value: char) -> Result<()> {
        self.0.push(value);
        Ok(())
    }

    fn serialize_str(self, value: &str) -> Result<()> {
        self.add_to_buffer(value);
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        for byte in value {
            self.serialize_str(&format!("{byte:02X}"))?;
        }
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.0.push('-');
        Ok(())
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(self, _: &'static str, _: u32, variant: &'static str) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()> {
        self.serialize_str(variant)?;
        self.ensure_white_space();
        value.serialize(self)
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self> {
        Ok(self)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self> {
        Ok(self)
    }

    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        _: usize,
    ) -> Result<Self> {
        self.serialize_str(variant)?;
        Ok(self)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self> {
        Ok(self)
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        _: usize,
    ) -> Result<Self> {
        self.serialize_str(variant)?;
        Ok(self)
    }
}

impl SerializeSeq for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        self.ensure_new_line();
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.ensure_new_line();
        self.0.push('\n');
        Ok(())
    }
}

impl SerializeTuple for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        self.ensure_white_space();
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl SerializeTupleStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        self.ensure_white_space();
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl SerializeTupleVariant for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        self.ensure_white_space();
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl SerializeMap for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<()> {
        self.ensure_new_line();
        key.serialize(&mut **self)
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        self.0.push(' ');
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.ensure_new_line();
        self.0.push('\n');
        Ok(())
    }
}

impl SerializeStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _: &'static str, value: &T) -> Result<()> {
        self.ensure_white_space();
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl SerializeStructVariant for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _: &'static str, value: &T) -> Result<()> {
        self.ensure_white_space();
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[derive(Serialize)]
    struct TupleStruct(i32, i32, i32);

    #[derive(Serialize)]
    enum Enum {
        Unit,
        Tuple(i32, i32, i32),
        Struct { a: i32, b: i32, c: i32 },
    }

    #[derive(Serialize)]
    struct Struct {
        a: i32,
        b: i32,
        c: i32,
    }

    #[derive(Serialize)]
    struct AdvancedStruct {
        a: i32,
        b: String,
        c: Vec<Struct>,
        d: BTreeMap<u64, Enum>,
        e: (u64, u8, Struct),
    }

    #[test]
    fn serialize_bool_true() {
        let mut serializer = Serializer::default();
        serializer.serialize_bool(true).unwrap();
        assert_eq!(serializer.0, "true");
    }

    #[test]
    fn serialize_bool_false() {
        let mut serializer = Serializer::default();
        serializer.serialize_bool(false).unwrap();
        assert_eq!(serializer.0, "false");
    }

    #[test]
    fn serialize_i8() {
        let mut serializer = Serializer::default();
        serializer.serialize_i8(1).unwrap();
        assert_eq!(serializer.0, "1");
    }

    #[test]
    fn serialize_i16() {
        let mut serializer = Serializer::default();
        serializer.serialize_i16(1).unwrap();
        assert_eq!(serializer.0, "1");
    }

    #[test]
    fn serialize_i32() {
        let mut serializer = Serializer::default();
        serializer.serialize_i32(1).unwrap();
        assert_eq!(serializer.0, "1");
    }

    #[test]
    fn serialize_i64() {
        let mut serializer = Serializer::default();
        serializer.serialize_i64(1).unwrap();
        assert_eq!(serializer.0, "1");
    }

    #[test]
    fn serialize_i128() {
        let mut serializer = Serializer::default();
        serializer.serialize_i128(1).unwrap();
        assert_eq!(serializer.0, "1");
    }

    #[test]
    fn serialize_u8() {
        let mut serializer = Serializer::default();
        serializer.serialize_u8(1).unwrap();
        assert_eq!(serializer.0, "1");
    }

    #[test]
    fn serialize_u16() {
        let mut serializer = Serializer::default();
        serializer.serialize_u16(1).unwrap();
        assert_eq!(serializer.0, "1");
    }

    #[test]
    fn serialize_u32() {
        let mut serializer = Serializer::default();
        serializer.serialize_u32(1).unwrap();
        assert_eq!(serializer.0, "1");
    }

    #[test]
    fn serialize_u64() {
        let mut serializer = Serializer::default();
        serializer.serialize_u64(1).unwrap();
        assert_eq!(serializer.0, "1");
    }

    #[test]
    fn serialize_u128() {
        let mut serializer = Serializer::default();
        serializer.serialize_u128(1).unwrap();
        assert_eq!(serializer.0, "1");
    }

    #[test]
    fn serialize_f32() {
        let mut serializer = Serializer::default();
        serializer.serialize_f32(1.5).unwrap();
        assert_eq!(serializer.0, "1.5");
    }

    #[test]
    fn serialize_f64() {
        let mut serializer = Serializer::default();
        serializer.serialize_f64(1.5).unwrap();
        assert_eq!(serializer.0, "1.5");
    }

    #[test]
    fn serialize_char() {
        let mut serializer = Serializer::default();
        serializer.serialize_char('a').unwrap();
        assert_eq!(serializer.0, "a");
    }

    #[test]
    fn serialize_str() {
        let mut serializer = Serializer::default();
        serializer.serialize_str("ab").unwrap();
        assert_eq!(serializer.0, "ab");
    }

    #[test]
    fn serialize_bytes() {
        let mut serializer = Serializer::default();
        serializer.serialize_bytes(&[0x01, 0x02, 0xab]).unwrap();
        assert_eq!(serializer.0, "0102AB");
    }

    #[test]
    fn serialize_none() {
        let mut serializer = Serializer::default();
        serializer.serialize_none().unwrap();
        assert_eq!(serializer.0, "-");
    }

    #[test]
    fn serialize_some() {
        let mut serializer = Serializer::default();
        serializer.serialize_some(&1).unwrap();
        assert_eq!(serializer.0, "1");
    }

    #[test]
    fn serialize_unit() {
        let mut serializer = Serializer::default();
        serializer.serialize_unit().unwrap();
        assert_eq!(serializer.0, "-");
    }

    #[test]
    fn serialize_unit_struct() {
        let mut serializer = Serializer::default();
        serializer.serialize_unit_struct("Unit").unwrap();
        assert_eq!(serializer.0, "-");
    }

    #[test]
    fn serialize_newtype_struct() {
        let mut serializer = Serializer::default();
        serializer.serialize_newtype_struct("Newtype", &1).unwrap();
        assert_eq!(serializer.0, "1");
    }

    #[test]
    fn serialize_newtype_variant() {
        let mut serializer = Serializer::default();
        serializer
            .serialize_newtype_variant("Newtype", 0, "Variant", &1)
            .unwrap();
        assert_eq!(serializer.0, "Variant 1");
    }

    #[test]
    fn serialize_seq() {
        let mut serializer = Serializer::default();
        let sequence = vec![1, 2, 3];
        Serialize::serialize(&sequence, &mut serializer).unwrap();
        assert_eq!(serializer.0, "1\n2\n3\n\n");
    }

    #[test]
    fn serialize_tuple() {
        let mut serializer = Serializer::default();
        let tuple = (1, 2, 3);
        Serialize::serialize(&tuple, &mut serializer).unwrap();
        assert_eq!(serializer.0, "1 2 3");
    }

    #[test]
    fn serialize_tuple_struct() {
        let mut serializer = Serializer::default();
        let tuple_struct = TupleStruct(1, 2, 3);
        Serialize::serialize(&tuple_struct, &mut serializer).unwrap();
        assert_eq!(serializer.0, "1 2 3");
    }

    #[test]
    fn serialize_unit_variant() {
        let mut serializer = Serializer::default();
        let unit_variant = Enum::Unit;
        Serialize::serialize(&unit_variant, &mut serializer).unwrap();
        assert_eq!(serializer.0, "Unit");
    }

    #[test]
    fn serialize_tuple_variant() {
        let mut serializer = Serializer::default();
        let tuple_variant = Enum::Tuple(1, 2, 3);
        Serialize::serialize(&tuple_variant, &mut serializer).unwrap();
        assert_eq!(serializer.0, "Tuple 1 2 3");
    }

    #[test]
    fn serialize_map() {
        let mut serializer = Serializer::default();
        let mut map = BTreeMap::new();
        map.insert(1, 2);
        map.insert(3, 4);
        Serialize::serialize(&map, &mut serializer).unwrap();
        assert_eq!(serializer.0, "1 2\n3 4\n\n");
    }

    #[test]
    fn serialize_struct() {
        let mut serializer = Serializer::default();
        let struct_ = Struct { a: 1, b: 2, c: 3 };
        Serialize::serialize(&struct_, &mut serializer).unwrap();
        assert_eq!(serializer.0, "1 2 3");
    }

    #[test]
    fn serialize_struct_variant() {
        let mut serializer = Serializer::default();
        let struct_variant = Enum::Struct { a: 1, b: 2, c: 3 };
        Serialize::serialize(&struct_variant, &mut serializer).unwrap();
        assert_eq!(serializer.0, "Struct 1 2 3");
    }

    #[test]
    fn serialize_advanced_struct() {
        let mut serializer = Serializer::default();
        let advanced_struct = AdvancedStruct {
            a: 1,
            b: "2".to_string(),
            c: vec![Struct { a: 3, b: 4, c: 5 }, Struct { a: 6, b: 7, c: 8 }],
            d: {
                let mut map = BTreeMap::new();
                map.insert(9, Enum::Unit);
                map.insert(10, Enum::Tuple(11, 12, 13));
                map
            },
            e: (14, 15, Struct { a: 16, b: 17, c: 18 }),
        };
        Serialize::serialize(&advanced_struct, &mut serializer).unwrap();
        assert_eq!(
            serializer.0,
            "1 2\n3 4 5\n6 7 8\n\n9 Unit\n10 Tuple 11 12 13\n\n14 15 16 17 18"
        );
    }
}
