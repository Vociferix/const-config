use serde::{
    Deserialize, Deserializer,
    de::{EnumAccess, Error, MapAccess, SeqAccess, VariantAccess, Visitor},
};

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    UInt(u64),
    Int(i64),
    Float(f64),
    Time(Time),
    DateTime(DateTime),
    Str(String),
    Bytes(Vec<u8>),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>),
    Map(Vec<(Value, Value)>),
}

#[derive(Debug, Clone, Copy)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub nanosecond: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct DateTime {
    pub date: Date,
    pub time: Option<OffsetTime>,
}

#[derive(Debug, Clone, Copy)]
pub struct OffsetTime {
    pub time: Time,
    pub offset_minutes: Option<i16>,
}

struct ValueVisitor;

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(ValueVisitor)
    }
}

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("any generically deserializable value")
    }

    fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(Value::Bool(v))
    }

    fn visit_i8<E: Error>(self, v: i8) -> Result<Self::Value, E> {
        Ok(Value::Int(v as i64))
    }

    fn visit_i16<E: Error>(self, v: i16) -> Result<Self::Value, E> {
        Ok(Value::Int(v as i64))
    }

    fn visit_i32<E: Error>(self, v: i32) -> Result<Self::Value, E> {
        Ok(Value::Int(v as i64))
    }

    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(Value::Int(v))
    }

    fn visit_i128<E: Error>(self, v: i128) -> Result<Self::Value, E> {
        if v < (i64::MIN as i128) || v > (i64::MAX as i128) {
            struct Msg(i128);

            impl std::fmt::Display for Msg {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "i128 value {} too large to deserialize", self.0)
                }
            }

            Err(E::custom(Msg(v)))
        } else {
            Ok(Value::Int(v as i64))
        }
    }

    fn visit_u8<E: Error>(self, v: u8) -> Result<Self::Value, E> {
        Ok(Value::UInt(v as u64))
    }

    fn visit_u16<E: Error>(self, v: u16) -> Result<Self::Value, E> {
        Ok(Value::UInt(v as u64))
    }

    fn visit_u32<E: Error>(self, v: u32) -> Result<Self::Value, E> {
        Ok(Value::UInt(v as u64))
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(Value::UInt(v))
    }

    fn visit_u128<E: Error>(self, v: u128) -> Result<Self::Value, E> {
        if v > (u64::MAX as u128) {
            struct Msg(u128);

            impl std::fmt::Display for Msg {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "u128 value {} too large to deserialize", self.0)
                }
            }

            Err(E::custom(Msg(v)))
        } else {
            Ok(Value::UInt(v as u64))
        }
    }

    fn visit_f32<E: Error>(self, v: f32) -> Result<Self::Value, E> {
        Ok(Value::Float(v as f64))
    }

    fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> {
        Ok(Value::Float(v))
    }

    fn visit_char<E: Error>(self, v: char) -> Result<Self::Value, E> {
        let mut buf = [0u8; 4];
        Ok(Value::Str(String::from(v.encode_utf8(&mut buf))))
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(Value::Str(String::from(v)))
    }

    fn visit_borrowed_str<E: Error>(self, v: &'de str) -> Result<Self::Value, E> {
        Ok(Value::Str(String::from(v)))
    }

    fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
        Ok(Value::Str(v))
    }

    fn visit_bytes<E: Error>(self, v: &[u8]) -> Result<Self::Value, E> {
        Ok(Value::Bytes(Vec::from(v)))
    }

    fn visit_borrowed_bytes<E: Error>(self, v: &'de [u8]) -> Result<Self::Value, E> {
        Ok(Value::Bytes(Vec::from(v)))
    }

    fn visit_byte_buf<E: Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
        Ok(Value::Bytes(v))
    }

    fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
        Ok(Value::Array(Vec::new()))
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut elems = if let Some(sz) = seq.size_hint() {
            Vec::with_capacity(sz)
        } else {
            Vec::new()
        };

        while let Some(elem) = seq.next_element::<Value>()? {
            elems.push(elem);
        }

        Ok(Value::Array(elems))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut entries = if let Some(sz) = map.size_hint() {
            Vec::with_capacity(sz)
        } else {
            Vec::new()
        };

        let mut is_obj = true;
        while let Some((key, value)) = map.next_entry::<Value, Value>()? {
            if !matches!(key, Value::Str(_)) {
                is_obj = false;
            }
            entries.push((key, value))
        }

        if is_obj {
            let mut obj = Vec::with_capacity(entries.len());
            for (key, value) in entries {
                let Value::Str(key) = key else {
                    unreachable!();
                };
                obj.push((key, value));
            }
            Ok(Value::Object(obj))
        } else {
            Ok(Value::Map(entries))
        }
    }
}
