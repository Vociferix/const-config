#![no_std]

#[cfg(feature = "serde")]
extern crate alloc;

use core::fmt::{self, Display};

#[cfg(feature = "serde")]
use alloc::{format, string::ToString};

#[derive(Debug, Clone, Copy, Default)]
pub struct DateParseError;

#[derive(Debug, Clone, Copy, Default)]
pub struct TimeParseError;

#[derive(Debug, Clone, Copy, Default)]
pub struct DateTimeParseError;

/// Build config from inline TOML text.
///
/// [`from_toml`] will parse TOML text a compile time and generate a
/// `const` [`Value`].
///
/// # Example
/// ```
/// use const_config::{Value, from_toml};
///
/// const CONFIG: Value<'static> = from_toml!(r#"
/// [table]
/// field1 = 42
/// field2 = "hello"
/// field3 = [1, 2, 3, 4]
/// "#);
/// ```
#[cfg(feature = "toml")]
pub use const_config_macros::from_toml;

/// Build config from a separate TOML file.
///
/// [`include_toml`] accepts a path relative to the directory containing
/// the file where the macro is called.
///
/// # Example
/// Given the following crate structure:
/// ```text
/// ├── Cargo.toml
/// ├── src
/// │   └── main.rs
/// └── configs
///     └── config.toml
/// ```
///
/// [`include_toml`] can be used to build a config constant from
/// `configs/config.toml` as shown below:
/// ```ignore
/// // src/main.rs
///
/// use const_config::{Value, include_toml};
///
/// const CONFIG: Value<'static> = include_toml!("../configs/config.toml");
///
/// fn main() {}
/// ```
#[cfg(feature = "toml")]
pub use const_config_macros::include_toml;

/// Build config from inline JSON text.
///
/// [`from_json`] will parse JSON text a compile time and generate a
/// `const` [`Value`].
///
/// # Example
/// ```
/// use const_config::{Value, from_json};
///
/// const CONFIG: Value<'static> = from_json!(r#"
/// {
///     "field1": 42,
///     "field2": "hello",
///     "field3": [1, 2, 3, 4]
/// }
/// "#);
/// ```
#[cfg(feature = "json")]
pub use const_config_macros::from_json;

/// Build config from a separate JSON file.
///
/// [`include_json`] accepts a path relative to the directory containing
/// the file where the macro is called.
///
/// # Example
/// Given the following crate structure:
/// ```text
/// ├── Cargo.toml
/// ├── src
/// │   └── main.rs
/// └── configs
///     └── config.json
/// ```
///
/// [`include_json`] can be used to build a config constant from
/// `configs/config.json` as shown below:
/// ```ignore
/// // src/main.rs
///
/// use const_config::{Value, include_json};
///
/// const CONFIG: Value<'static> = include_json!("../configs/config.json");
///
/// fn main() {}
/// ```
#[cfg(feature = "json")]
pub use const_config_macros::include_json;

/// Build config from inline YAML text.
///
/// [`from_yaml`] will parse YAML text a compile time and generate a
/// `const` [`Value`].
///
/// # Example
/// ```
/// use const_config::{Value, from_yaml};
///
/// const CONFIG: Value<'static> = from_yaml!(r#"
/// ---
/// field1: 42
/// field2: hello
/// field3:
///   - 1
///   - 2
///   - 3
///   - 4
/// "#);
/// ```
#[cfg(feature = "yaml")]
pub use const_config_macros::from_yaml;

/// Build config from a separate YAML file.
///
/// [`include_yaml`] accepts a path relative to the directory containing
/// the file where the macro is called.
///
/// # Example
/// Given the following crate structure:
/// ```text
/// ├── Cargo.toml
/// ├── src
/// │   └── main.rs
/// └── configs
///     └── config.yml
/// ```
///
/// [`include_yaml`] can be used to build a config constant from
/// `configs/config.yml` as shown below:
/// ```ignore
/// // src/main.rs
///
/// use const_config::{Value, include_yaml};
///
/// const CONFIG: Value<'static> = include_yaml!("../configs/config.yml");
///
/// fn main() {}
/// ```
#[cfg(feature = "yaml")]
pub use const_config_macros::include_yaml;

#[cfg(feature = "ron")]
pub use const_config_macros::from_ron;

#[cfg(feature = "ron")]
pub use const_config_macros::include_ron;

#[cfg(feature = "json5")]
pub use const_config_macros::from_json5;

#[cfg(feature = "json5")]
pub use const_config_macros::include_json5;

#[cfg(feature = "s-expr")]
pub use const_config_macros::from_s_expr;

#[cfg(feature = "s-expr")]
pub use const_config_macros::include_s_expr;

#[cfg(feature = "hjson")]
pub use const_config_macros::from_hjson;

#[cfg(feature = "hjson")]
pub use const_config_macros::include_hjson;

#[cfg(feature = "csv")]
pub use const_config_macros::from_csv;

#[cfg(feature = "csv")]
pub use const_config_macros::include_csv;

#[cfg(feature = "cbor")]
pub use const_config_macros::from_cbor;

#[cfg(feature = "cbor")]
pub use const_config_macros::include_cbor;

#[cfg(feature = "msgpack")]
pub use const_config_macros::from_msgpack;

#[cfg(feature = "msgpack")]
pub use const_config_macros::include_msgpack;

#[cfg(feature = "pickle")]
pub use const_config_macros::from_pickle;

#[cfg(feature = "pickle")]
pub use const_config_macros::include_pickle;

#[cfg(feature = "bson")]
pub use const_config_macros::from_bson;

#[cfg(feature = "bson")]
pub use const_config_macros::include_bson;

#[cfg(feature = "flexbuffers")]
pub use const_config_macros::from_flexbuffers;

#[cfg(feature = "flexbuffers")]
pub use const_config_macros::include_flexbuffers;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value<'a> {
    Null,
    Bool(bool),
    Number(Number),
    Date(Date),
    Time(Time),
    DateTime(DateTime),
    Str(&'a str),
    Bytes(&'a [u8]),
    Array(&'a [Value<'a>]),
    Object(Object<'a>),
    Map(Map<'a>),
}

#[derive(Debug, Clone, Copy)]
pub enum Number {
    UInt(u128),
    Int(i128),
    Float(f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
    pub offset: Option<i16>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub nanosecond: u32,
}

#[derive(Clone, Copy)]
pub struct Object<'a> {
    entries: &'a [(&'a str, Value<'a>)],
    phf_params: &'a [u32],
    phf_values: &'a [u32],
}

#[derive(Clone, Copy)]
pub struct Map<'a> {
    entries: &'a [(Value<'a>, Value<'a>)],
    phf_params: &'a [u32],
    phf_values: &'a [u32],
}

impl<'a> core::fmt::Debug for Object<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Debug::fmt(self.entries, f)
    }
}

impl<'a> core::fmt::Debug for Map<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Debug::fmt(self.entries, f)
    }
}

const fn jenkins_mix(mut value: u32) -> u32 {
    value = value.wrapping_add(0x7ed55d16).wrapping_add(value << 12);
    value = (value ^ 0xc761c23c) ^ (value >> 19);
    value = value.wrapping_add(0x165667b1).wrapping_add(value << 5);
    value = value.wrapping_add(0xd3a2646c) ^ (value << 9);
    value = value.wrapping_add(0xfd7046c5).wrapping_add(value << 3);
    value = (value ^ 0xb55a4f09) ^ (value >> 16);
    value
}

const fn jenkins_hash(init: u32, value: &[u8]) -> u32 {
    let mut h = init;
    let mut idx = 0usize;
    while idx < value.len() {
        h = h.wrapping_add(value[idx] as u32);
        h = h.wrapping_add(h << 10);
        h ^= h >> 6;
        idx += 1;
    }
    h = h.wrapping_add(h << 3);
    h ^= h >> 11;
    h = h.wrapping_add(h << 15);
    h
}

const fn hash_combine<const COUNT: usize>(hashes: [u32; COUNT]) -> u32 {
    let mut h = hashes[0];
    let mut idx = 1usize;
    while idx < COUNT {
        h ^= hashes[idx]
            .wrapping_add(0x9e3779b9)
            .wrapping_add(h << 6)
            .wrapping_add(h >> 2);
        idx += 1;
    }
    h
}

const fn bytes_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut idx = 0usize;
    while idx < a.len() {
        if a[idx] != b[idx] {
            return false;
        }
        idx += 1;
    }
    true
}

const fn str_eq(a: &str, b: &str) -> bool {
    bytes_eq(a.as_bytes(), b.as_bytes())
}

impl<'a> Object<'a> {
    #[doc(hidden)]
    pub const fn new<const LEN: usize>(
        entries: &'a [(&'a str, Value); LEN],
        phf_params: &'a [u32; LEN],
        phf_values: &'a [u32; LEN],
    ) -> Self {
        Self {
            entries,
            phf_params,
            phf_values,
        }
    }

    const fn hash(param: u32, key: &str) -> usize {
        jenkins_hash(jenkins_mix(param), key.as_bytes()) as usize
    }

    const fn index(&self, key: &str) -> usize {
        let modulus = self.entries.len();
        let i = Self::hash(0, key) % modulus;
        let i = Self::hash(self.phf_params[i], key) % modulus;
        self.phf_values[i] as usize
    }

    pub const fn try_get(&self, key: &str) -> Option<&'a Value> {
        if self.entries.is_empty() {
            return None;
        }

        let entry = &self.entries[self.index(key)];
        if str_eq(key, entry.0) {
            Some(&entry.1)
        } else {
            None
        }
    }

    pub const fn get(&self, key: &str) -> &'a Value {
        self.try_get(key).unwrap()
    }

    pub const fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub const fn len(&self) -> usize {
        self.entries.len()
    }

    pub const fn entries(&self) -> &'a [(&'a str, Value)] {
        self.entries
    }

    pub const fn contains(&self, key: &str) -> bool {
        self.try_get(key).is_some()
    }

    pub const fn copy(&self) -> Self {
        Self {
            entries: self.entries,
            phf_params: self.phf_params,
            phf_values: self.phf_values,
        }
    }
}

impl<'a> PartialEq for Object<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for (key, value) in self.entries {
            if let Some(other_value) = other.try_get(key) {
                if *value != *other_value {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

impl<'a> Map<'a> {
    #[doc(hidden)]
    pub const fn new<const LEN: usize>(
        entries: &'a [(Value, Value); LEN],
        phf_params: &'a [u32; LEN],
        phf_values: &'a [u32; LEN],
    ) -> Self {
        Self {
            entries,
            phf_params,
            phf_values,
        }
    }

    const fn hash(param: u32, key: &Value) -> u32 {
        let h = jenkins_mix(param);
        match key {
            Value::Null => hash_combine([0, h]),
            Value::Bool(false) => hash_combine([1, h]),
            Value::Bool(true) => hash_combine([2, h]),
            Value::Number(Number::UInt(val)) => hash_combine([
                3,
                h,
                (*val >> 96) as u32,
                ((*val >> 64) & 0xffffffff) as u32,
                ((*val >> 32) & 0xffffffff) as u32,
                ((*val & 0xffffffff) as u32),
            ]),
            Value::Number(Number::Int(val)) => {
                let seed = if *val < 0 { 4 } else { 3 };
                let val = u128::from_ne_bytes((*val).to_ne_bytes());
                hash_combine([
                    seed,
                    h,
                    (val >> 96) as u32,
                    ((val >> 64) & 0xffffffff) as u32,
                    ((val >> 32) & 0xffffffff) as u32,
                    ((val & 0xffffffff) as u32),
                ])
            }
            Value::Number(Number::Float(val)) => {
                if *val < 0.0f64 {
                    if ((*val as i128) as f64) == *val {
                        let val = u128::from_ne_bytes((*val as i128).to_ne_bytes());
                        hash_combine([
                            4,
                            h,
                            (val >> 96) as u32,
                            ((val >> 64) & 0xffffffff) as u32,
                            ((val >> 32) & 0xffffffff) as u32,
                            ((val & 0xffffffff) as u32),
                        ])
                    } else {
                        let val = u64::from_ne_bytes((*val).to_ne_bytes());
                        hash_combine([5, h, (val >> 32) as u32, (val & 0xffffffff) as u32])
                    }
                } else {
                    if ((*val as u128) as f64) == *val {
                        let val = *val as u128;
                        hash_combine([
                            3,
                            h,
                            (val >> 96) as u32,
                            ((val >> 64) & 0xffffffff) as u32,
                            ((val >> 32) & 0xffffffff) as u32,
                            ((val & 0xffffffff) as u32),
                        ])
                    } else {
                        let val = u64::from_ne_bytes((*val).to_ne_bytes());
                        hash_combine([5, h, (val >> 32) as u32, (val & 0xffffffff) as u32])
                    }
                }
            }
            Value::Date(date) => {
                hash_combine([6, date.year as u32, date.month as u32, date.day as u32])
            }
            Value::Time(time) => hash_combine([
                7,
                h,
                time.hour as u32,
                time.minute as u32,
                time.second as u32,
                time.nanosecond,
            ]),
            Value::DateTime(dt) => {
                let h = hash_combine([
                    8,
                    h,
                    dt.date.year as u32,
                    dt.date.month as u32,
                    dt.date.day as u32,
                    dt.time.hour as u32,
                    dt.time.minute as u32,
                    dt.time.second as u32,
                    dt.time.nanosecond,
                ]);
                if let Some(offset) = &dt.offset {
                    hash_combine([h, 1, u16::from_ne_bytes((*offset).to_ne_bytes()) as u32])
                } else {
                    hash_combine([h, 0])
                }
            }
            Value::Str(s) => hash_combine([9, jenkins_hash(h, s.as_bytes())]),
            Value::Bytes(b) => hash_combine([10, jenkins_hash(h, b)]),
            Value::Array(array) => {
                let mut h = hash_combine([11, h]);
                let mut idx = 0usize;
                while idx < array.len() {
                    h = Self::hash(h, &array[idx]);
                    idx += 1;
                }
                h
            }
            Value::Object(obj) => {
                let mut h = hash_combine([12, h]);
                let mut idx = 0usize;
                while idx < obj.len() {
                    let entry = &obj.entries[idx];
                    h = jenkins_hash(h, entry.0.as_bytes());
                    h = Self::hash(h, &entry.1);
                    idx += 1;
                }
                h
            }
            Value::Map(map) => {
                let mut h = hash_combine([13, h]);
                let mut idx = 0usize;
                while idx < map.len() {
                    let entry = &map.entries[idx];
                    h = Self::hash(h, &entry.0);
                    h = Self::hash(h, &entry.1);
                    idx += 1;
                }
                h
            }
        }
    }

    const fn key_eq(lhs: &Value, rhs: &Value) -> bool {
        match (lhs, rhs) {
            (Value::Null, Value::Null) => true,
            (Value::Bool(lhs), Value::Bool(rhs)) => *lhs == *rhs,
            (Value::Number(lhs), Value::Number(rhs)) => match (lhs, rhs) {
                (Number::UInt(lhs), Number::UInt(rhs)) => *lhs == *rhs,
                (Number::Int(lhs), Number::Int(rhs)) => *lhs == *rhs,
                (Number::Float(lhs), Number::Float(rhs)) => *lhs == *rhs,
                (Number::UInt(lhs), Number::Int(rhs)) => *rhs >= 0 && *lhs == (*rhs as u128),
                (Number::Int(lhs), Number::UInt(rhs)) => *lhs >= 0 && (*lhs as u128) == *rhs,
                (Number::UInt(lhs), Number::Float(rhs)) => {
                    *rhs >= 0.0f64 && (*rhs as u128 as f64) == *rhs && *lhs == (*rhs as u128)
                }
                (Number::Float(lhs), Number::UInt(rhs)) => {
                    *lhs >= 0.0f64 && (*lhs as u128 as f64) == *lhs && (*lhs as u128) == *rhs
                }
                (Number::Int(lhs), Number::Float(rhs)) => {
                    (*rhs as i128 as f64) == *rhs && *lhs == (*rhs as i128)
                }
                (Number::Float(lhs), Number::Int(rhs)) => {
                    (*lhs as i128 as f64) == *lhs && (*lhs as i128) == *rhs
                }
            },
            (Value::Date(lhs), Value::Date(rhs)) => {
                lhs.year == rhs.year && lhs.month == rhs.month && lhs.day == rhs.day
            }
            (Value::Time(lhs), Value::Time(rhs)) => {
                lhs.hour == rhs.hour
                    && lhs.minute == rhs.minute
                    && lhs.second == rhs.second
                    && lhs.nanosecond == rhs.nanosecond
            }
            (Value::DateTime(lhs), Value::DateTime(rhs)) => {
                lhs.date.year == rhs.date.year
                    && lhs.date.month == rhs.date.month
                    && lhs.date.day == rhs.date.day
                    && lhs.time.hour == rhs.time.hour
                    && lhs.time.minute == rhs.time.minute
                    && lhs.time.second == rhs.time.second
                    && lhs.time.nanosecond == rhs.time.nanosecond
                    && match (&lhs.offset, &rhs.offset) {
                        (Some(l), Some(r)) => *l == *r,
                        (None, None) => true,
                        _ => false,
                    }
            }
            (Value::Str(lhs), Value::Str(rhs)) => str_eq(*lhs, *rhs),
            (Value::Bytes(lhs), Value::Bytes(rhs)) => bytes_eq(*lhs, *rhs),
            (Value::Array(lhs), Value::Array(rhs)) => {
                lhs.len() == rhs.len() && {
                    let mut ret = true;
                    let mut idx = 0usize;
                    while idx < lhs.len() {
                        if !Self::key_eq(&lhs[idx], &rhs[idx]) {
                            ret = false;
                            break;
                        }
                        idx += 1;
                    }
                    ret
                }
            }
            (Value::Object(lhs), Value::Object(rhs)) => {
                lhs.len() == rhs.len() && {
                    let mut ret = true;
                    let mut idx = 0usize;
                    while idx < lhs.len() {
                        if let Some(val) = rhs.try_get(lhs.entries[idx].0) {
                            if !Self::key_eq(&lhs.entries[idx].1, val) {
                                ret = false;
                                break;
                            }
                        } else {
                            ret = false;
                            break;
                        }
                        idx += 1;
                    }
                    ret
                }
            }
            (Value::Map(lhs), Value::Map(rhs)) => {
                lhs.len() == rhs.len() && {
                    let mut ret = true;
                    let mut idx = 0usize;
                    while idx < lhs.len() {
                        if let Some(val) = rhs.try_get(&lhs.entries[idx].0) {
                            if !Self::key_eq(&lhs.entries[idx].1, val) {
                                ret = false;
                                break;
                            }
                        } else {
                            ret = false;
                            break;
                        }
                        idx += 1;
                    }
                    ret
                }
            }
            _ => false,
        }
    }

    const fn index(&self, key: &Value) -> usize {
        let modulus = self.entries.len();
        let i = (Self::hash(0, key) as usize) % modulus;
        let i = (Self::hash(self.phf_params[i], key) as usize) % modulus;
        self.phf_values[i] as usize
    }

    pub const fn try_get(&self, key: &Value) -> Option<&'a Value<'a>> {
        if self.entries.is_empty() {
            return None;
        }

        let entry = &self.entries[self.index(key)];
        if !Self::key_eq(key, &entry.0) {
            return None;
        }

        Some(&entry.1)
    }

    pub const fn try_get_bool(&self, key: bool) -> Option<&'a Value<'a>> {
        self.try_get(&Value::Bool(key))
    }

    pub const fn try_get_number(&self, key: Number) -> Option<&'a Value<'a>> {
        self.try_get(&Value::Number(key))
    }

    pub const fn try_get_u8(&self, key: u8) -> Option<&'a Value<'a>> {
        self.try_get_u128(key as u128)
    }

    pub const fn try_get_u16(&self, key: u16) -> Option<&'a Value<'a>> {
        self.try_get_u128(key as u128)
    }

    pub const fn try_get_u32(&self, key: u32) -> Option<&'a Value<'a>> {
        self.try_get_u128(key as u128)
    }

    pub const fn try_get_u64(&self, key: u64) -> Option<&'a Value<'a>> {
        self.try_get_u128(key as u128)
    }

    pub const fn try_get_u128(&self, key: u128) -> Option<&'a Value<'a>> {
        self.try_get_number(Number::UInt(key))
    }

    pub const fn try_get_i8(&self, key: i8) -> Option<&'a Value<'a>> {
        self.try_get_i128(key as i128)
    }

    pub const fn try_get_i16(&self, key: i16) -> Option<&'a Value<'a>> {
        self.try_get_i128(key as i128)
    }

    pub const fn try_get_i32(&self, key: i32) -> Option<&'a Value<'a>> {
        self.try_get_i128(key as i128)
    }

    pub const fn try_get_i64(&self, key: i64) -> Option<&'a Value<'a>> {
        self.try_get_i128(key as i128)
    }

    pub const fn try_get_i128(&self, key: i128) -> Option<&'a Value<'a>> {
        self.try_get_number(Number::Int(key))
    }

    pub const fn try_get_f32(&self, key: f32) -> Option<&'a Value<'a>> {
        self.try_get_f64(key as f64)
    }

    pub const fn try_get_f64(&self, key: f64) -> Option<&'a Value<'a>> {
        self.try_get_number(Number::Float(key))
    }

    pub const fn try_get_char(&self, key: char) -> Option<&'a Value<'a>> {
        if let Some(value) = self.try_get_u32(key as u32) {
            Some(value)
        } else {
            let mut buf = [0u8; 4];
            self.try_get_str(key.encode_utf8(&mut buf))
        }
    }

    pub const fn try_get_str(&self, key: &str) -> Option<&'a Value<'a>> {
        self.try_get(&Value::Str(key))
    }

    pub const fn try_get_bytes(&self, key: &[u8]) -> Option<&'a Value<'a>> {
        self.try_get(&Value::Bytes(key))
    }

    pub const fn try_get_date(&self, key: Date) -> Option<&'a Value<'a>> {
        self.try_get(&Value::Date(key))
    }

    pub const fn try_get_time(&self, key: Time) -> Option<&'a Value<'a>> {
        self.try_get(&Value::Time(key))
    }

    pub const fn try_get_datetime(&self, key: DateTime) -> Option<&'a Value<'a>> {
        self.try_get(&Value::DateTime(key))
    }

    pub const fn get(&self, key: &Value) -> &'a Value<'a> {
        self.try_get(key).unwrap()
    }

    pub const fn get_bool(&self, key: bool) -> &'a Value<'a> {
        self.try_get_bool(key).unwrap()
    }

    pub const fn get_number(&self, key: Number) -> &'a Value<'a> {
        self.try_get_number(key).unwrap()
    }

    pub const fn get_u8(&self, key: u8) -> &'a Value<'a> {
        self.try_get_u8(key).unwrap()
    }

    pub const fn get_u16(&self, key: u16) -> &'a Value<'a> {
        self.try_get_u16(key).unwrap()
    }

    pub const fn get_u32(&self, key: u32) -> &'a Value<'a> {
        self.try_get_u32(key).unwrap()
    }

    pub const fn get_u64(&self, key: u64) -> &'a Value<'a> {
        self.try_get_u64(key).unwrap()
    }

    pub const fn get_u128(&self, key: u128) -> &'a Value<'a> {
        self.try_get_u128(key).unwrap()
    }

    pub const fn get_i8(&self, key: i8) -> &'a Value<'a> {
        self.try_get_i8(key).unwrap()
    }

    pub const fn get_i16(&self, key: i16) -> &'a Value<'a> {
        self.try_get_i16(key).unwrap()
    }

    pub const fn get_i32(&self, key: i32) -> &'a Value<'a> {
        self.try_get_i32(key).unwrap()
    }

    pub const fn get_i64(&self, key: i64) -> &'a Value<'a> {
        self.try_get_i64(key).unwrap()
    }

    pub const fn get_i128(&self, key: i128) -> &'a Value<'a> {
        self.try_get_i128(key).unwrap()
    }

    pub const fn get_f32(&self, key: f32) -> &'a Value<'a> {
        self.try_get_f32(key).unwrap()
    }

    pub const fn get_f64(&self, key: f64) -> &'a Value<'a> {
        self.try_get_f64(key).unwrap()
    }

    pub const fn get_char(&self, key: char) -> &'a Value<'a> {
        self.try_get_char(key).unwrap()
    }

    pub const fn get_str(&self, key: &str) -> &'a Value<'a> {
        self.try_get_str(key).unwrap()
    }

    pub const fn get_bytes(&self, key: &[u8]) -> &'a Value<'a> {
        self.try_get_bytes(key).unwrap()
    }

    pub const fn get_time(&self, key: Time) -> &'a Value<'a> {
        self.try_get_time(key).unwrap()
    }

    pub const fn get_datetime(&self, key: DateTime) -> &'a Value<'a> {
        self.try_get_datetime(key).unwrap()
    }

    pub const fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub const fn len(&self) -> usize {
        self.entries.len()
    }

    pub const fn entries(&self) -> &'a [(Value<'a>, Value<'a>)] {
        self.entries
    }

    pub const fn contains(&self, key: &Value) -> bool {
        self.try_get(key).is_some()
    }

    pub const fn contains_bool(&self, key: bool) -> bool {
        self.try_get_bool(key).is_some()
    }

    pub const fn contains_number(&self, key: Number) -> bool {
        self.try_get_number(key).is_some()
    }

    pub const fn contains_u8(&self, key: u8) -> bool {
        self.try_get_u8(key).is_some()
    }

    pub const fn contains_u16(&self, key: u16) -> bool {
        self.try_get_u16(key).is_some()
    }

    pub const fn contains_u32(&self, key: u32) -> bool {
        self.try_get_u32(key).is_some()
    }

    pub const fn contains_u64(&self, key: u64) -> bool {
        self.try_get_u64(key).is_some()
    }

    pub const fn contains_u128(&self, key: u128) -> bool {
        self.try_get_u128(key).is_some()
    }

    pub const fn contains_i8(&self, key: i8) -> bool {
        self.try_get_i8(key).is_some()
    }

    pub const fn contains_i16(&self, key: i16) -> bool {
        self.try_get_i16(key).is_some()
    }

    pub const fn contains_i32(&self, key: i32) -> bool {
        self.try_get_i32(key).is_some()
    }

    pub const fn contains_i64(&self, key: i64) -> bool {
        self.try_get_i64(key).is_some()
    }

    pub const fn contains_i128(&self, key: i128) -> bool {
        self.try_get_i128(key).is_some()
    }

    pub const fn contains_f32(&self, key: f32) -> bool {
        self.try_get_f32(key).is_some()
    }

    pub const fn contains_f64(&self, key: f64) -> bool {
        self.try_get_f64(key).is_some()
    }

    pub const fn contains_char(&self, key: char) -> bool {
        self.try_get_char(key).is_some()
    }

    pub const fn contains_str(&self, key: &str) -> bool {
        self.try_get_str(key).is_some()
    }

    pub const fn contains_time(&self, key: Time) -> bool {
        self.try_get_time(key).is_some()
    }

    pub const fn contains_datetime(&self, key: DateTime) -> bool {
        self.try_get_datetime(key).is_some()
    }

    pub const fn copy(&self) -> Self {
        Self {
            entries: self.entries,
            phf_params: self.phf_params,
            phf_values: self.phf_values,
        }
    }
}

impl<'a> PartialEq for Map<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for (key, value) in self.entries {
            if let Some(other_value) = other.try_get(key) {
                if *value != *other_value {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

impl Number {
    pub const fn is_uint(&self) -> bool {
        matches!(self, Self::UInt(_))
    }

    pub const fn is_int(&self) -> bool {
        matches!(self, Self::Int(_))
    }

    pub const fn is_integral(&self) -> bool {
        return !self.is_float();
    }

    pub const fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    pub const fn try_as_u8(&self) -> Option<u8> {
        match self {
            Self::UInt(value) if *value <= (u8::MAX as u128) => Some(*value as u8),
            Self::Int(value) if *value >= 0 && *value <= (u8::MAX as i128) => Some(*value as u8),
            Self::Float(value) if (*value as u8 as f64) == *value => Some(*value as u8),
            _ => None,
        }
    }

    pub const fn try_as_u16(&self) -> Option<u16> {
        match self {
            Self::UInt(value) if *value <= (u16::MAX as u128) => Some(*value as u16),
            Self::Int(value) if *value >= 0 && *value <= (u16::MAX as i128) => Some(*value as u16),
            Self::Float(value) if (*value as u16 as f64) == *value => Some(*value as u16),
            _ => None,
        }
    }

    pub const fn try_as_u32(&self) -> Option<u32> {
        match self {
            Self::UInt(value) if *value <= (u32::MAX as u128) => Some(*value as u32),
            Self::Int(value) if *value >= 0 && *value <= (u32::MAX as i128) => Some(*value as u32),
            Self::Float(value) if (*value as u32 as f64) == *value => Some(*value as u32),
            _ => None,
        }
    }

    pub const fn try_as_u64(&self) -> Option<u64> {
        match self {
            Self::UInt(value) if *value <= (u64::MAX as u128) => Some(*value as u64),
            Self::Int(value) if *value >= 0 && *value <= (u64::MAX as i128) => Some(*value as u64),
            Self::Float(value) if (*value as u64 as f64) == *value => Some(*value as u64),
            _ => None,
        }
    }

    pub const fn try_as_u128(&self) -> Option<u128> {
        match self {
            Self::UInt(value) => Some(*value),
            Self::Int(value) if *value >= 0 => Some(*value as u128),
            Self::Float(value) if (*value as u128 as f64) == *value => Some(*value as u128),
            _ => None,
        }
    }

    pub const fn try_as_i8(&self) -> Option<i8> {
        match self {
            Self::UInt(value) if *value < (i8::MAX as u128) => Some(*value as i8),
            Self::Int(value) if *value >= (i8::MIN as i128) && *value <= (i8::MAX as i128) => {
                Some(*value as i8)
            }
            Self::Float(value) if (*value as i8 as f64) == *value => Some(*value as i8),
            _ => None,
        }
    }

    pub const fn try_as_i16(&self) -> Option<i16> {
        match self {
            Self::UInt(value) if *value < (i16::MAX as u128) => Some(*value as i16),
            Self::Int(value) if *value >= (i16::MIN as i128) && *value <= (i16::MAX as i128) => {
                Some(*value as i16)
            }
            Self::Float(value) if (*value as i16 as f64) == *value => Some(*value as i16),
            _ => None,
        }
    }

    pub const fn try_as_i32(&self) -> Option<i32> {
        match self {
            Self::UInt(value) if *value < (i32::MAX as u128) => Some(*value as i32),
            Self::Int(value) if *value >= (i32::MIN as i128) && *value <= (i32::MAX as i128) => {
                Some(*value as i32)
            }
            Self::Float(value) if (*value as i32 as f64) == *value => Some(*value as i32),
            _ => None,
        }
    }

    pub const fn try_as_i64(&self) -> Option<i64> {
        match self {
            Self::UInt(value) if *value < (i64::MAX as u128) => Some(*value as i64),
            Self::Int(value) if *value >= (i64::MIN as i128) && *value <= (i64::MAX as i128) => {
                Some(*value as i64)
            }
            Self::Float(value) if (*value as i64 as f64) == *value => Some(*value as i64),
            _ => None,
        }
    }

    pub const fn try_as_i128(&self) -> Option<i128> {
        match self {
            Self::UInt(value) if *value <= (i128::MAX as u128) => Some(*value as i128),
            Self::Int(value) => Some(*value),
            Self::Float(value) if (*value as i128 as f64) == *value => Some(*value as i128),
            _ => None,
        }
    }

    pub const fn try_as_char(&self) -> Option<char> {
        if let Some(value) = self.try_as_u32() {
            char::from_u32(value)
        } else {
            None
        }
    }

    pub const fn as_u8(&self) -> u8 {
        self.try_as_u8().unwrap()
    }

    pub const fn as_u16(&self) -> u16 {
        self.try_as_u16().unwrap()
    }

    pub const fn as_u32(&self) -> u32 {
        self.try_as_u32().unwrap()
    }

    pub const fn as_u64(&self) -> u64 {
        self.try_as_u64().unwrap()
    }

    pub const fn as_u128(&self) -> u128 {
        self.try_as_u128().unwrap()
    }

    pub const fn as_i8(&self) -> i8 {
        self.try_as_i8().unwrap()
    }

    pub const fn as_i16(&self) -> i16 {
        self.try_as_i16().unwrap()
    }

    pub const fn as_i32(&self) -> i32 {
        self.try_as_i32().unwrap()
    }

    pub const fn as_i64(&self) -> i64 {
        self.try_as_i64().unwrap()
    }

    pub const fn as_i128(&self) -> i128 {
        self.try_as_i128().unwrap()
    }

    pub const fn as_f32(&self) -> f32 {
        match self {
            Self::UInt(value) => *value as f32,
            Self::Int(value) => *value as f32,
            Self::Float(value) => *value as f32,
        }
    }

    pub const fn as_f64(&self) -> f64 {
        match self {
            Self::UInt(value) => *value as f64,
            Self::Int(value) => *value as f64,
            Self::Float(value) => *value,
        }
    }

    pub const fn as_char(&self) -> char {
        self.try_as_char().unwrap()
    }

    pub const fn copy(&self) -> Self {
        match self {
            Self::UInt(value) => Self::UInt(*value),
            Self::Int(value) => Self::Int(*value),
            Self::Float(value) => Self::Float(*value),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::UInt(lhs), Self::UInt(rhs)) => *lhs == *rhs,
            (Self::Int(lhs), Self::Int(rhs)) => *lhs == *rhs,
            (Self::Float(lhs), Self::Float(rhs)) => *lhs == *rhs,
            (Self::UInt(lhs), Self::Int(rhs)) => *rhs >= 0 && *lhs == (*rhs as u128),
            (Self::Int(lhs), Self::UInt(rhs)) => *lhs >= 0 && (*lhs as u128) == *rhs,
            (Self::UInt(lhs), Self::Float(rhs)) => {
                *rhs >= 0.0f64 && (*rhs as u128 as f64) == *rhs && *lhs == (*rhs as u128)
            }
            (Self::Float(lhs), Self::UInt(rhs)) => {
                *lhs >= 0.0f64 && (*lhs as u128 as f64) == *lhs && (*lhs as u128) == *rhs
            }
            (Self::Int(lhs), Self::Float(rhs)) => {
                (*rhs as i128 as f64) == *rhs && *lhs == (*rhs as i128)
            }
            (Self::Float(lhs), Self::Int(rhs)) => {
                (*lhs as i128 as f64) == *lhs && (*lhs as i128) == *rhs
            }
        }
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match (self, other) {
            (Self::UInt(lhs), Self::UInt(rhs)) => PartialOrd::partial_cmp(lhs, rhs),
            (Self::Int(lhs), Self::Int(rhs)) => PartialOrd::partial_cmp(lhs, rhs),
            (Self::Float(lhs), Self::Float(rhs)) => PartialOrd::partial_cmp(lhs, rhs),
            (Self::UInt(lhs), Self::Int(rhs)) => {
                if *rhs < 0 {
                    Some(core::cmp::Ordering::Greater)
                } else {
                    PartialOrd::partial_cmp(lhs, &(*rhs as u128))
                }
            }
            (Self::Int(lhs), Self::UInt(rhs)) => {
                if *lhs < 0 {
                    Some(core::cmp::Ordering::Less)
                } else {
                    PartialOrd::partial_cmp(&(*lhs as u128), rhs)
                }
            }
            (Self::UInt(lhs), Self::Float(rhs)) => {
                if *rhs < 0.0f64 {
                    Some(core::cmp::Ordering::Greater)
                } else if (*rhs as u128 as f64) == *rhs {
                    PartialOrd::partial_cmp(lhs, &(*rhs as u128))
                } else {
                    match PartialOrd::partial_cmp(&(*lhs as f64), rhs) {
                        Some(core::cmp::Ordering::Equal) => Some(core::cmp::Ordering::Greater),
                        ord => ord,
                    }
                }
            }
            (Self::Float(lhs), Self::UInt(rhs)) => {
                if *lhs < 0.0f64 {
                    Some(core::cmp::Ordering::Less)
                } else if (*lhs as u128 as f64) == *lhs {
                    PartialOrd::partial_cmp(&(*lhs as u128), rhs)
                } else {
                    match PartialOrd::partial_cmp(lhs, &(*rhs as f64)) {
                        Some(core::cmp::Ordering::Equal) => Some(core::cmp::Ordering::Less),
                        ord => ord,
                    }
                }
            }
            (Self::Int(lhs), Self::Float(rhs)) => {
                if (*rhs as i128 as f64) == *rhs {
                    PartialOrd::partial_cmp(lhs, &(*rhs as i128))
                } else {
                    match PartialOrd::partial_cmp(&(*lhs as f64), rhs) {
                        Some(core::cmp::Ordering::Equal) => Some(core::cmp::Ordering::Greater),
                        ord => ord,
                    }
                }
            }
            (Self::Float(lhs), Self::Int(rhs)) => {
                if (*lhs as i128 as f64) == *lhs {
                    PartialOrd::partial_cmp(&(*lhs as i128), rhs)
                } else {
                    match PartialOrd::partial_cmp(lhs, &(*rhs as f64)) {
                        Some(core::cmp::Ordering::Equal) => Some(core::cmp::Ordering::Less),
                        ord => ord,
                    }
                }
            }
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UInt(value) => Display::fmt(value, f),
            Self::Int(value) => Display::fmt(value, f),
            Self::Float(value) => Display::fmt(value, f),
        }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl Display for DateParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("invalid date string, expected date in YYYY-MM-DD format")
    }
}

/* poor man's nom parsers below :D */

fn parse_symb(s: &[u8], symb: u8) -> Option<((), &[u8])> {
    let Some((b, rem)) = s.split_first() else {
        return None;
    };
    if *b != symb {
        return None;
    }
    Some(((), rem))
}

fn parse_digit(s: &[u8]) -> Option<(u8, &[u8])> {
    let Some((digit, rem)) = s.split_first() else {
        return None;
    };
    if *digit < b'0' || *digit > b'9' {
        return None;
    }
    Some((*digit - b'0', rem))
}

fn parse_2_digit_num(s: &[u8]) -> Option<(u8, &[u8])> {
    let (digit1, s) = parse_digit(s)?;
    let (digit2, s) = parse_digit(s)?;
    Some((digit1 * 10 + digit2, s))
}

fn parse_4_digit_num(s: &[u8]) -> Option<(u16, &[u8])> {
    let (half1, s) = parse_2_digit_num(s)?;
    let (half2, s) = parse_2_digit_num(s)?;
    Some(((half1 as u16) * 100 + (half2 as u16), s))
}

fn parse_date(s: &[u8]) -> Option<(Date, &[u8])> {
    let (year, s) = parse_4_digit_num(s)?;
    let (_, s) = parse_symb(s, b'-')?;
    let (month, s) = parse_2_digit_num(s)?;
    let (_, s) = parse_symb(s, b'-')?;
    let (day, s) = parse_2_digit_num(s)?;
    Some((Date { year, month, day }, s))
}

fn parse_time(s: &[u8]) -> Option<(Time, &[u8])> {
    let (hour, s) = parse_2_digit_num(s)?;
    let (_, s) = parse_symb(s, b':')?;
    let (minute, s) = parse_2_digit_num(s)?;
    let (_, s) = parse_symb(s, b':')?;
    let (second, s) = parse_2_digit_num(s)?;
    let (nanosecond, s) = if let Some((_, s)) = parse_symb(s, b'.') {
        let mut s = s;
        let mut nano = 0u32;
        let mut count = 0usize;
        loop {
            if let Some((digit, rem)) = parse_digit(s) {
                count += 1;
                if count <= 9 {
                    nano = (nano * 10) + (digit as u32);
                }
                s = rem;
            } else {
                break;
            }
        }
        while count < 9 {
            nano *= 10;
            count += 1;
        }
        (nano, s)
    } else {
        (0u32, s)
    };
    Some((
        Time {
            hour,
            minute,
            second,
            nanosecond,
        },
        s,
    ))
}

fn parse_offset(s: &[u8]) -> Option<(i16, &[u8])> {
    let Some((symb, s)) = s.split_first() else {
        return None;
    };
    if *symb == b'Z' {
        return Some((0, s));
    }
    let neg = if *symb == b'+' {
        false
    } else if *symb == b'-' {
        true
    } else {
        return None;
    };
    let (hour, s) = parse_2_digit_num(s)?;
    let (_, s) = parse_symb(s, b':')?;
    let (minute, s) = parse_2_digit_num(s)?;
    let offset = ((hour as i16) * 60) + (minute as i16);
    let offset = if neg { -offset } else { offset };
    Some((offset, s))
}

fn parse_datetime(s: &[u8]) -> Option<(DateTime, &[u8])> {
    let (date, s) = parse_date(s)?;
    let (_, s) = parse_symb(s, b'T')?;
    let (time, s) = parse_time(s)?;
    let (offset, s) = if let Some((offset, s)) = parse_offset(s) {
        (Some(offset), s)
    } else {
        (None, s)
    };
    Some((DateTime { date, time, offset }, s))
}

impl core::str::FromStr for Date {
    type Err = DateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((date, rem)) = parse_date(s.as_bytes()) {
            if rem.is_empty() {
                Ok(date)
            } else {
                Err(DateParseError)
            }
        } else {
            Err(DateParseError)
        }
    }
}

impl Date {
    #[cfg(feature = "chrono")]
    pub const fn as_naive_date(&self) -> chrono::NaiveDate {
        chrono::NaiveDate::from_ymd_opt(self.year as i32, self.month as u32, self.day as u32)
            .expect("Invalid date")
    }

    pub const fn copy(&self) -> Self {
        Self {
            year: self.year,
            month: self.month,
            day: self.day,
        }
    }
}

#[cfg(feature = "chrono")]
impl From<Date> for chrono::NaiveDate {
    fn from(date: Date) -> Self {
        date.as_naive_date()
    }
}

#[cfg(feature = "chrono")]
impl<'a> From<&'a Date> for chrono::NaiveDate {
    fn from(date: &'a Date) -> Self {
        date.as_naive_date()
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)?;
        if self.nanosecond != 0 {
            if self.nanosecond % 1_000_000 == 0 {
                write!(f, ".{:03}", self.nanosecond / 1_000_000)?;
            } else if self.nanosecond % 1_000 == 0 {
                write!(f, ".{:06}", self.nanosecond / 1_000)?;
            } else {
                write!(f, ".{:09}", self.nanosecond)?;
            }
        }
        Ok(())
    }
}

impl Display for TimeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("invalid time string, expected time in HH:MM:SS[.fff] format")
    }
}

impl core::str::FromStr for Time {
    type Err = TimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((time, rem)) = parse_time(s.as_bytes()) {
            if rem.is_empty() {
                Ok(time)
            } else {
                Err(TimeParseError)
            }
        } else {
            Err(TimeParseError)
        }
    }
}

impl Time {
    #[cfg(feature = "chrono")]
    pub const fn as_naive_time(&self) -> chrono::NaiveTime {
        chrono::NaiveTime::from_hms_nano_opt(
            self.hour as u32,
            self.minute as u32,
            self.second as u32,
            self.nanosecond as u32,
        )
        .expect("Invalid time")
    }

    pub const fn copy(&self) -> Self {
        Self {
            hour: self.hour,
            minute: self.minute,
            second: self.second,
            nanosecond: self.nanosecond,
        }
    }
}

#[cfg(feature = "chrono")]
impl From<Time> for chrono::NaiveTime {
    fn from(time: Time) -> Self {
        time.as_naive_time()
    }
}

#[cfg(feature = "chrono")]
impl<'a> From<&'a Time> for chrono::NaiveTime {
    fn from(time: &'a Time) -> Self {
        time.as_naive_time()
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match Ord::cmp(&self.offset, &other.offset) {
            core::cmp::Ordering::Equal => {}
            _ => {
                return None;
            }
        }

        match Ord::cmp(&self.date, &other.date) {
            core::cmp::Ordering::Equal => {}
            ord => {
                return Some(ord);
            }
        }

        Some(Ord::cmp(&self.time, &other.time))
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}T{}", self.date, self.time)?;
        match self.offset {
            Some(0) => f.write_str("Z"),
            Some(offset) => {
                let offset = if offset < 0 {
                    f.write_str("-")?;
                    (-(offset as i32)) as u32
                } else {
                    f.write_str("+")?;
                    offset as u32
                };

                let hours = offset / 60;
                let minutes = offset % 60;
                write!(f, "{:02}:{:02}", hours, minutes)
            }
            None => Ok(()),
        }
    }
}

impl core::str::FromStr for DateTime {
    type Err = DateTimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((dt, rem)) = parse_datetime(s.as_bytes()) {
            if rem.is_empty() {
                Ok(dt)
            } else {
                Err(DateTimeParseError)
            }
        } else {
            Err(DateTimeParseError)
        }
    }
}

impl DateTime {
    #[cfg(feature = "chrono")]
    pub const fn as_naive_datetime_and_offset(
        &self,
    ) -> (chrono::NaiveDateTime, Option<chrono::FixedOffset>) {
        (
            chrono::NaiveDateTime::new(self.date.as_naive_date(), self.time.as_naive_time()),
            if let Some(offset) = &self.offset {
                Some(chrono::FixedOffset::east_opt(*offset as i32 * 60).unwrap())
            } else {
                None
            },
        )
    }

    pub const fn copy(&self) -> Self {
        Self {
            date: self.date.copy(),
            time: self.time.copy(),
            offset: if let Some(offset) = &self.offset {
                Some(*offset)
            } else {
                None
            },
        }
    }
}

#[cfg(feature = "chrono")]
impl<Tz> From<DateTime> for chrono::DateTime<Tz>
where
    Tz: chrono::TimeZone<Offset = chrono::FixedOffset>,
{
    fn from(dt: DateTime) -> Self {
        let (dt, offset) = dt.as_naive_datetime_and_offset();
        if let Some(offset) = offset {
            chrono::DateTime::from_naive_utc_and_offset(dt, offset)
        } else {
            chrono::DateTime::from_naive_utc_and_offset(
                dt,
                chrono::FixedOffset::east_opt(0).unwrap(),
            )
        }
    }
}

#[cfg(feature = "chrono")]
impl<'a, Tz> From<&'a DateTime> for chrono::DateTime<Tz>
where
    Tz: chrono::TimeZone<Offset = chrono::FixedOffset>,
{
    fn from(dt: &'a DateTime) -> Self {
        let (dt, offset) = dt.as_naive_datetime_and_offset();
        if let Some(offset) = offset {
            chrono::DateTime::from_naive_utc_and_offset(dt, offset)
        } else {
            chrono::DateTime::from_naive_utc_and_offset(
                dt,
                chrono::FixedOffset::east_opt(0).unwrap(),
            )
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Self::UInt(value) => serializer.serialize_u128(value),
            Self::Int(value) => serializer.serialize_i128(value),
            Self::Float(value) => serializer.serialize_f64(value),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

#[cfg(feature = "serde")]
impl<'a> serde::Serialize for Object<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_map(self.entries.iter().copied())
    }
}

#[cfg(feature = "serde")]
impl<'a> serde::Serialize for Map<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_map(self.entries.iter().copied())
    }
}

#[cfg(feature = "serde")]
impl<'a> serde::Serialize for Value<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeTuple;

        match self {
            Self::Null => serializer.serialize_none(),
            Self::Bool(value) => serializer.serialize_bool(*value),
            Self::Number(value) => serde::Serialize::serialize(value, serializer),
            Self::Date(value) => serde::Serialize::serialize(value, serializer),
            Self::Time(value) => serde::Serialize::serialize(value, serializer),
            Self::DateTime(value) => serde::Serialize::serialize(value, serializer),
            Self::Str(value) => serializer.serialize_str(value),
            Self::Bytes(value) => serializer.serialize_bytes(value),
            Self::Array(value) => {
                if value.is_empty() {
                    serializer.serialize_unit()
                } else {
                    let mut ser = serializer.serialize_tuple(value.len())?;
                    for elem in *value {
                        ser.serialize_element(elem)?;
                    }
                    ser.end()
                }
            }
            Self::Object(value) => serde::Serialize::serialize(value, serializer),
            Self::Map(value) => serde::Serialize::serialize(value, serializer),
        }
    }
}

impl<'a> Value<'a> {
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub const fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    pub const fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    pub const fn is_uint(&self) -> bool {
        matches!(self, Self::Number(Number::UInt(_)))
    }

    pub const fn is_int(&self) -> bool {
        matches!(self, Self::Number(Number::Int(_)))
    }

    pub const fn is_integral(&self) -> bool {
        matches!(self, Self::Number(Number::UInt(_) | Number::Int(_)))
    }

    pub const fn is_float(&self) -> bool {
        matches!(self, Self::Number(Number::Float(_)))
    }

    pub const fn is_date(&self) -> bool {
        matches!(self, Self::Date(_))
    }

    pub const fn is_time(&self) -> bool {
        matches!(self, Self::Time(_))
    }

    pub const fn is_datetime(&self) -> bool {
        matches!(self, Self::DateTime(_))
    }

    pub const fn is_str(&self) -> bool {
        matches!(self, Self::Str(_))
    }

    pub const fn is_bytes(&self) -> bool {
        matches!(self, Self::Bytes(_))
    }

    pub const fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub const fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    pub const fn try_as_unit(&self) -> Option<()> {
        match self {
            Value::Array(value) if value.is_empty() => Some(()),
            Value::Object(value) if value.is_empty() => Some(()),
            _ => None,
        }
    }

    pub const fn try_as_bool(&self) -> Option<bool> {
        if let Value::Bool(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub const fn try_as_number(&self) -> Option<Number> {
        if let Value::Number(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub const fn try_as_u8(&self) -> Option<u8> {
        if let Value::Number(value) = self {
            value.try_as_u8()
        } else {
            None
        }
    }

    pub const fn try_as_u16(&self) -> Option<u16> {
        if let Value::Number(value) = self {
            value.try_as_u16()
        } else {
            None
        }
    }

    pub const fn try_as_u32(&self) -> Option<u32> {
        if let Value::Number(value) = self {
            value.try_as_u32()
        } else {
            None
        }
    }

    pub const fn try_as_u64(&self) -> Option<u64> {
        if let Value::Number(value) = self {
            value.try_as_u64()
        } else {
            None
        }
    }

    pub const fn try_as_u128(&self) -> Option<u128> {
        if let Value::Number(value) = self {
            value.try_as_u128()
        } else {
            None
        }
    }

    pub const fn try_as_i8(&self) -> Option<i8> {
        if let Value::Number(value) = self {
            value.try_as_i8()
        } else {
            None
        }
    }

    pub const fn try_as_i16(&self) -> Option<i16> {
        if let Value::Number(value) = self {
            value.try_as_i16()
        } else {
            None
        }
    }

    pub const fn try_as_i32(&self) -> Option<i32> {
        if let Value::Number(value) = self {
            value.try_as_i32()
        } else {
            None
        }
    }

    pub const fn try_as_i64(&self) -> Option<i64> {
        if let Value::Number(value) = self {
            value.try_as_i64()
        } else {
            None
        }
    }

    pub const fn try_as_i128(&self) -> Option<i128> {
        if let Value::Number(value) = self {
            value.try_as_i128()
        } else {
            None
        }
    }

    pub const fn try_as_f32(&self) -> Option<f32> {
        if let Value::Number(value) = self {
            Some(value.as_f32())
        } else {
            None
        }
    }

    pub const fn try_as_f64(&self) -> Option<f64> {
        if let Value::Number(value) = self {
            Some(value.as_f64())
        } else {
            None
        }
    }

    pub const fn try_as_char(&self) -> Option<char> {
        match self {
            Value::Number(value) => value.try_as_char(),
            Value::Str(value) if !value.is_empty() => {
                let s = value.as_bytes();
                let codepoint = if s[0] < 0x80 {
                    if s.len() > 1 {
                        return None;
                    }
                    s[0] as u32
                } else if s[1] < 0b1110_0000 {
                    if s.len() > 2 {
                        return None;
                    }
                    (((s[0] & 0b0001_1111) as u32) << 6) | ((s[1] & 0b0011_1111) as u32)
                } else if s[2] < 0b1111_0000 {
                    if s.len() > 3 {
                        return None;
                    }
                    (((s[0] & 0b0000_1111) as u32) << 12)
                        | (((s[1] & 0b0011_1111) as u32) << 6)
                        | ((s[2] & 0b0011_1111) as u32)
                } else {
                    if s.len() > 4 {
                        return None;
                    }
                    (((s[0] & 0b0000_1111) as u32) << 18)
                        | (((s[1] & 0b0011_1111) as u32) << 12)
                        | (((s[2] & 0b0011_1111) as u32) << 6)
                        | ((s[3] & 0b0011_1111) as u32)
                };
                unsafe { Some(char::from_u32_unchecked(codepoint)) }
            }
            _ => None,
        }
    }

    pub const fn try_as_date(&self) -> Option<Date> {
        if let Value::Date(date) = self {
            Some(date.copy())
        } else {
            None
        }
    }

    #[cfg(feature = "chrono")]
    pub const fn try_as_naive_date(&self) -> Option<chrono::NaiveDate> {
        if let Value::Date(date) = self {
            Some(date.as_naive_date())
        } else {
            None
        }
    }

    pub const fn try_as_time(&self) -> Option<Time> {
        if let Value::Time(time) = self {
            Some(time.copy())
        } else {
            None
        }
    }

    #[cfg(feature = "chrono")]
    pub const fn try_as_naive_time(&self) -> Option<chrono::NaiveTime> {
        if let Value::Time(time) = self {
            Some(time.as_naive_time())
        } else {
            None
        }
    }

    pub const fn try_as_datetime(&self) -> Option<DateTime> {
        if let Value::DateTime(datetime) = self {
            Some(datetime.copy())
        } else {
            None
        }
    }

    #[cfg(feature = "chrono")]
    pub const fn try_as_naive_datetime_and_offset(
        &self,
    ) -> Option<(chrono::NaiveDateTime, Option<chrono::FixedOffset>)> {
        if let Value::DateTime(datetime) = self {
            Some(datetime.as_naive_datetime_and_offset())
        } else {
            None
        }
    }

    #[cfg(feature = "chrono")]
    pub fn try_as_datetime_tz<Tz>(&self) -> Option<chrono::DateTime<Tz>>
    where
        Tz: chrono::TimeZone<Offset = chrono::FixedOffset>,
    {
        if let Value::DateTime(datetime) = self {
            Some(datetime.into())
        } else {
            None
        }
    }

    pub const fn try_as_str(&self) -> Option<&'a str> {
        if let Value::Str(s) = self {
            Some(s)
        } else if let Value::Bytes(b) = self {
            if let Ok(s) = core::str::from_utf8(b) {
                Some(s)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub const fn try_as_bytes(&self) -> Option<&'a [u8]> {
        if let Value::Bytes(b) = self {
            Some(b)
        } else if let Value::Str(s) = self {
            Some(s.as_bytes())
        } else {
            None
        }
    }

    pub const fn try_as_array(&self) -> Option<&'a [Value<'a>]> {
        if let Value::Array(array) = self {
            Some(array)
        } else {
            None
        }
    }

    pub const fn try_as_object(&self) -> Option<Object<'a>> {
        if let Value::Object(obj) = self {
            Some(obj.copy())
        } else {
            None
        }
    }

    pub const fn try_as_map(&self) -> Option<Map<'a>> {
        if let Value::Map(map) = self {
            Some(map.copy())
        } else {
            None
        }
    }

    pub const fn as_unit(&self) -> () {
        self.try_as_unit().unwrap()
    }

    pub const fn as_bool(&self) -> bool {
        self.try_as_bool().unwrap()
    }

    pub const fn as_number(&self) -> Number {
        self.try_as_number().unwrap()
    }

    pub const fn as_u8(&self) -> u8 {
        self.try_as_u8().unwrap()
    }

    pub const fn as_u16(&self) -> u16 {
        self.try_as_u16().unwrap()
    }

    pub const fn as_u32(&self) -> u32 {
        self.try_as_u32().unwrap()
    }

    pub const fn as_u64(&self) -> u64 {
        self.try_as_u64().unwrap()
    }

    pub const fn as_u128(&self) -> u128 {
        self.try_as_u128().unwrap()
    }

    pub const fn as_i8(&self) -> i8 {
        self.try_as_i8().unwrap()
    }

    pub const fn as_i16(&self) -> i16 {
        self.try_as_i16().unwrap()
    }

    pub const fn as_i32(&self) -> i32 {
        self.try_as_i32().unwrap()
    }

    pub const fn as_i64(&self) -> i64 {
        self.try_as_i64().unwrap()
    }

    pub const fn as_i128(&self) -> i128 {
        self.try_as_i128().unwrap()
    }

    pub const fn as_f32(&self) -> f32 {
        self.try_as_f32().unwrap()
    }

    pub const fn as_f64(&self) -> f64 {
        self.try_as_f64().unwrap()
    }

    pub const fn as_char(&self) -> char {
        self.try_as_char().unwrap()
    }

    pub const fn as_date(&self) -> Date {
        self.try_as_date().unwrap()
    }

    #[cfg(feature = "chrono")]
    pub const fn as_naive_date(&self) -> chrono::NaiveDate {
        self.try_as_naive_date().unwrap()
    }

    pub const fn as_time(&self) -> Time {
        self.try_as_time().unwrap()
    }

    #[cfg(feature = "chrono")]
    pub const fn as_naive_time(&self) -> chrono::NaiveTime {
        self.try_as_naive_time().unwrap()
    }

    pub const fn as_datetime(&self) -> DateTime {
        self.try_as_datetime().unwrap()
    }

    #[cfg(feature = "chrono")]
    pub const fn as_naive_datetime_and_offset(
        &self,
    ) -> (chrono::NaiveDateTime, Option<chrono::FixedOffset>) {
        self.try_as_naive_datetime_and_offset().unwrap()
    }

    #[cfg(feature = "chrono")]
    pub fn as_datetime_tz<Tz>(&self) -> chrono::DateTime<Tz>
    where
        Tz: chrono::TimeZone<Offset = chrono::FixedOffset>,
    {
        self.try_as_datetime_tz().unwrap()
    }

    pub const fn as_str(&self) -> &'a str {
        self.try_as_str().unwrap()
    }

    pub const fn as_bytes(&self) -> &'a [u8] {
        self.try_as_bytes().unwrap()
    }

    pub const fn as_array(&self) -> &'a [Value<'a>] {
        self.try_as_array().unwrap()
    }

    pub const fn as_object(&self) -> Object<'a> {
        self.try_as_object().unwrap()
    }

    pub const fn as_map(&self) -> Map<'a> {
        self.try_as_map().unwrap()
    }

    pub const fn copy(&self) -> Self {
        match self {
            Self::Null => Self::Null,
            Self::Bool(value) => Self::Bool(*value),
            Self::Number(value) => Self::Number(value.copy()),
            Self::Date(date) => Self::Date(date.copy()),
            Self::Time(value) => Self::Time(value.copy()),
            Self::DateTime(value) => Self::DateTime(value.copy()),
            Self::Str(value) => Self::Str(*value),
            Self::Bytes(value) => Self::Bytes(*value),
            Self::Array(value) => Self::Array(*value),
            Self::Object(value) => Self::Object(value.copy()),
            Self::Map(value) => Self::Map(value.copy()),
        }
    }

    #[cfg(feature = "serde")]
    pub fn try_interpret_as<D>(&self) -> Result<D, InterpretError>
    where
        D: serde::Deserialize<'a>,
    {
        D::deserialize(Deser(self.clone()))
    }

    #[cfg(feature = "serde")]
    pub fn interpret_as<D>(&self) -> D
    where
        D: serde::Deserialize<'a>,
    {
        self.try_interpret_as::<D>().unwrap()
    }
}

#[cfg(feature = "serde")]
#[derive(Debug, Clone)]
pub struct InterpretError(alloc::string::String);

#[cfg(feature = "serde")]
struct Deser<'a>(Value<'a>);

#[cfg(feature = "serde")]
struct DeserArray<'a>(&'a [Value<'a>]);

#[cfg(feature = "serde")]
struct DeserObject<'a>(&'a [(&'a str, Value<'a>)]);

#[cfg(feature = "serde")]
struct DeserMap<'a>(&'a [(Value<'a>, Value<'a>)]);

#[cfg(feature = "serde")]
struct DeserEnum<'a>(Value<'a>);

#[cfg(feature = "serde")]
struct DeserVariant<'a>(Option<&'a Value<'a>>);

#[cfg(feature = "serde")]
struct DeserKey<'a>(&'a str);

#[cfg(feature = "serde")]
impl Display for InterpretError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "const-config interpret error: {}", self.0)
    }
}

#[cfg(feature = "serde")]
impl core::error::Error for InterpretError {}

#[cfg(feature = "serde")]
impl serde::de::Error for InterpretError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        use alloc::string::ToString;
        InterpretError(msg.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'a> serde::de::SeqAccess<'a> for DeserArray<'a> {
    type Error = InterpretError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'a>,
    {
        if let Some((head, tail)) = self.0.split_first() {
            *self = DeserArray(tail);
            Ok(Some(seed.deserialize(Deser(head.clone()))?))
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.0.len())
    }
}

#[cfg(feature = "serde")]
impl<'a> serde::de::MapAccess<'a> for DeserObject<'a> {
    type Error = InterpretError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'a>,
    {
        if let Some(entry) = self.0.first() {
            Ok(Some(seed.deserialize(DeserKey(entry.0))?))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'a>,
    {
        if let Some((head, tail)) = self.0.split_first() {
            *self = DeserObject(tail);
            Ok(seed.deserialize(Deser(head.1.clone()))?)
        } else {
            Err(InterpretError(format!(
                "attempt to deserialize object field with no remaining fields to deserialize: {:?}",
                self.0
            )))
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.0.len())
    }
}

#[cfg(feature = "serde")]
impl<'a> serde::de::MapAccess<'a> for DeserMap<'a> {
    type Error = InterpretError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'a>,
    {
        if let Some(entry) = self.0.first() {
            Ok(Some(seed.deserialize(Deser(entry.0.clone()))?))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'a>,
    {
        if let Some((head, tail)) = self.0.split_first() {
            *self = DeserMap(tail);
            Ok(seed.deserialize(Deser(head.1.clone()))?)
        } else {
            Err(InterpretError(format!(
                "attempt to deserialize map entry value with no remaining entries to deserialize: {:?}",
                self.0
            )))
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.0.len())
    }
}

#[cfg(feature = "serde")]
impl<'a> serde::de::EnumAccess<'a> for DeserEnum<'a> {
    type Error = InterpretError;
    type Variant = DeserVariant<'a>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'a>,
    {
        match self.0 {
            Value::Str(_) => Ok((seed.deserialize(Deser(self.0.clone()))?, DeserVariant(None))),
            Value::Array(value) if value.len() == 1 => Ok((
                seed.deserialize(Deser(value[0].clone()))?,
                DeserVariant(None),
            )),
            Value::Array(value) if value.len() == 2 => Ok((
                seed.deserialize(Deser(value[0].clone()))?,
                DeserVariant(Some(&value[1])),
            )),
            Value::Object(value) if value.len() == 1 => {
                let entry = &value.entries[0];
                Ok((
                    seed.deserialize(DeserKey(entry.0))?,
                    DeserVariant(Some(&entry.1)),
                ))
            }
            _ => Err(InterpretError(format!(
                "attempt to deserialize enum from data that is incompatible with enum layout: {:?}",
                self.0
            ))),
        }
    }
}

#[cfg(feature = "serde")]
impl<'a> serde::de::VariantAccess<'a> for DeserVariant<'a> {
    type Error = InterpretError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        if self.0.is_none() {
            Ok(())
        } else {
            Err(InterpretError(format!(
                "expected no data for unit variant, but have data: {:?}",
                self.0
            )))
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'a>,
    {
        if let Some(value) = self.0 {
            seed.deserialize(Deser(value.clone()))
        } else {
            Err(InterpretError(format!(
                "expected data for newtype variant, but have none"
            )))
        }
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        use serde::Deserializer;

        if let Some(value) = self.0 {
            Deser(value.clone()).deserialize_tuple(len, visitor)
        } else {
            Err(InterpretError(format!(
                "expected data for tuple variant, but have none"
            )))
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        use serde::Deserializer;

        if let Some(value) = self.0 {
            Deser(value.clone()).deserialize_map(visitor)
        } else {
            Err(InterpretError(format!(
                "expected data for struct variant, but have none"
            )))
        }
    }
}

#[cfg(feature = "serde")]
impl<'a> serde::Deserializer<'a> for DeserKey<'a> {
    type Error = InterpretError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_borrowed_str(self.0)
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as bool, have key {:?}",
            self.0
        )))
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as i8, have key {:?}",
            self.0
        )))
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as i16, have key {:?}",
            self.0
        )))
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as i32, have key {:?}",
            self.0
        )))
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as i64, have key {:?}",
            self.0
        )))
    }

    fn deserialize_i128<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as i128, have key {:?}",
            self.0
        )))
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as u8, have key {:?}",
            self.0
        )))
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as u8, have key {:?}",
            self.0
        )))
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as u32, have key {:?}",
            self.0
        )))
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as u64, have key {:?}",
            self.0
        )))
    }

    fn deserialize_u128<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as u128, have key {:?}",
            self.0
        )))
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as f32, have key {:?}",
            self.0
        )))
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as f64, have key {:?}",
            self.0
        )))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        let mut it = self.0.chars();
        if let Some(ch) = it.next() {
            if it.next().is_none() {
                visitor.visit_char(ch)
            } else {
                Err(InterpretError(format!(
                    "attempt to deserialize multi-character string as single character: {:?}",
                    self.0
                )))
            }
        } else {
            Err(InterpretError(format!(
                "attempt to deserialize empty string as single character"
            )))
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_borrowed_str(self.0)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_string(self.0.into())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_borrowed_bytes(self.0.as_bytes())
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_byte_buf(self.0.as_bytes().into())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as unit, have key {:?}",
            self.0
        )))
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as unit struct, have key {:?}",
            self.0
        )))
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as newtype struct, have key {:?}",
            self.0
        )))
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as sequence, have key {:?}",
            self.0
        )))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as tuple, have key {:?}",
            self.0
        )))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as tuple struct, have key {:?}",
            self.0
        )))
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as map, have key {:?}",
            self.0
        )))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as struct, have key {:?}",
            self.0
        )))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(InterpretError(format!(
            "attempt to deserialize object key as enum, have key {:?}",
            self.0
        )))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_borrowed_str(self.0)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        self.deserialize_any(visitor)
    }

    fn is_human_readable(&self) -> bool {
        true
    }
}

#[cfg(feature = "serde")]
impl<'a> serde::Deserializer<'a> for Deser<'a> {
    type Error = InterpretError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.0 {
            Value::Null => visitor.visit_none(),
            Value::Bool(value) => visitor.visit_bool(value),
            Value::Number(Number::UInt(value)) => visitor.visit_u128(value),
            Value::Number(Number::Int(value)) => visitor.visit_i128(value),
            Value::Number(Number::Float(value)) => visitor.visit_f64(value),
            Value::Date(value) => visitor.visit_string(value.to_string()),
            Value::Time(value) => visitor.visit_string(value.to_string()),
            Value::DateTime(value) => visitor.visit_string(value.to_string()),
            Value::Str(value) => visitor.visit_str(value),
            Value::Bytes(value) => visitor.visit_bytes(value),
            Value::Array(value) => visitor.visit_seq(DeserArray(value)),
            Value::Object(value) => visitor.visit_map(DeserObject(value.entries)),
            Value::Map(value) => visitor.visit_map(DeserMap(value.entries)),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        if let Value::Bool(value) = self.0 {
            visitor.visit_bool(value)
        } else {
            Err(InterpretError(format!(
                "requested deserialization of bool, have {:?}",
                self.0
            )))
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_i8(self.0.try_as_i8().ok_or_else(|| {
            InterpretError(format!(
                "requested deserialization of i8, have {:?}",
                self.0
            ))
        })?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_i16(self.0.try_as_i16().ok_or_else(|| {
            InterpretError(format!(
                "requested deserialization of i16, have {:?}",
                self.0
            ))
        })?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_i32(self.0.try_as_i32().ok_or_else(|| {
            InterpretError(format!(
                "requested deserialization of i32, have {:?}",
                self.0
            ))
        })?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_i64(self.0.try_as_i64().ok_or_else(|| {
            InterpretError(format!(
                "requested deserialization of i64, have {:?}",
                self.0
            ))
        })?)
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_i128(self.0.try_as_i128().ok_or_else(|| {
            InterpretError(format!(
                "requested deserialization of i128, have {:?}",
                self.0
            ))
        })?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_u8(self.0.try_as_u8().ok_or_else(|| {
            InterpretError(format!(
                "requested deserialization of u8, have {:?}",
                self.0
            ))
        })?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_u16(self.0.try_as_u16().ok_or_else(|| {
            InterpretError(format!(
                "requested deserialization of u16, have {:?}",
                self.0
            ))
        })?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_u32(self.0.try_as_u32().ok_or_else(|| {
            InterpretError(format!(
                "requested deserialization of u32, have {:?}",
                self.0
            ))
        })?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_u64(self.0.try_as_u64().ok_or_else(|| {
            InterpretError(format!(
                "requested deserialization of u64, have {:?}",
                self.0
            ))
        })?)
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_u128(self.0.try_as_u128().ok_or_else(|| {
            InterpretError(format!(
                "requested deserialization of u128, have {:?}",
                self.0
            ))
        })?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_f32(self.0.try_as_f32().ok_or_else(|| {
            InterpretError(format!(
                "requested deserialization of f32, have {:?}",
                self.0
            ))
        })?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_f64(self.0.try_as_f64().ok_or_else(|| {
            InterpretError(format!(
                "requested deserialization of f64, have {:?}",
                self.0
            ))
        })?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_char(self.0.try_as_char().ok_or_else(|| {
            InterpretError(format!(
                "requested deserialization of char, have {:?}",
                self.0
            ))
        })?)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.0 {
            Value::Str(value) => visitor.visit_borrowed_str(value),
            Value::Date(value) => visitor.visit_string(value.to_string()),
            Value::Time(value) => visitor.visit_string(value.to_string()),
            Value::DateTime(value) => visitor.visit_string(value.to_string()),
            _ => Err(InterpretError(format!(
                "requested deserialization of str, have {:?}",
                self.0
            ))),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.0 {
            Value::Str(value) => visitor.visit_string((*value).into()),
            Value::Date(value) => visitor.visit_string(value.to_string()),
            Value::Time(value) => visitor.visit_string(value.to_string()),
            Value::DateTime(value) => visitor.visit_string(value.to_string()),
            _ => Err(InterpretError(format!(
                "requested deserialization of String, have {:?}",
                self.0
            ))),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.0 {
            Value::Str(value) => visitor.visit_borrowed_bytes(value.as_bytes()),
            Value::Array(value) => visitor.visit_seq(DeserArray(value)),
            _ => Err(InterpretError(format!(
                "requested deserialization of bytes, have {:?}",
                self.0
            ))),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.0 {
            Value::Str(value) => visitor.visit_byte_buf(value.as_bytes().into()),
            Value::Array(value) => {
                let mut bytes = alloc::vec::Vec::with_capacity(value.len());
                for val in value {
                    if let Some(v) = val.try_as_u8() {
                        bytes.push(v);
                    } else {
                        return Err(InterpretError(format!(
                            "requested deserialization of byte buffer, have {:?}",
                            self.0
                        )));
                    }
                }
                visitor.visit_byte_buf(bytes)
            }
            _ => Err(InterpretError(format!(
                "requested deserialization of byte buffer, have {:?}",
                self.0
            ))),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.0 {
            Value::Null => visitor.visit_none(),
            _ => visitor.visit_some(Deser(self.0)),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.0 {
            Value::Array(value) if value.is_empty() => visitor.visit_unit(),
            Value::Object(value) if value.is_empty() => visitor.visit_unit(),
            _ => Err(InterpretError(format!(
                "requested deserialization of unit, have {:?}",
                self.0
            ))),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        if let Value::Array(value) = self.0 {
            visitor.visit_seq(DeserArray(value))
        } else {
            Err(InterpretError(format!(
                "requested deserialization as sequence, have {:?}",
                self.0
            )))
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.0 {
            Value::Object(value) => visitor.visit_map(DeserObject(value.entries)),
            Value::Map(value) => visitor.visit_map(DeserMap(value.entries)),
            _ => Err(InterpretError(format!(
                "requested deserialization as map, have {:?}",
                self.0
            ))),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_enum(DeserEnum(self.0))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        self.deserialize_any(visitor)
    }

    fn is_human_readable(&self) -> bool {
        true
    }
}

#[cfg(feature = "serde")]
impl<'a> serde::Deserialize<'a> for Number {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        struct NumberVisitor;

        impl<'a> serde::de::Visitor<'a> for NumberVisitor {
            type Value = Number;

            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str("a numeric value")
            }

            fn visit_i8<E>(self, v: i8) -> Result<Number, E>
            where
                E: serde::de::Error,
            {
                Ok(Number::Int(v as i128))
            }

            fn visit_i16<E>(self, v: i16) -> Result<Number, E>
            where
                E: serde::de::Error,
            {
                Ok(Number::Int(v as i128))
            }

            fn visit_i32<E>(self, v: i32) -> Result<Number, E>
            where
                E: serde::de::Error,
            {
                Ok(Number::Int(v as i128))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Number, E>
            where
                E: serde::de::Error,
            {
                Ok(Number::Int(v as i128))
            }

            fn visit_i128<E>(self, v: i128) -> Result<Number, E>
            where
                E: serde::de::Error,
            {
                Ok(Number::Int(v))
            }

            fn visit_u8<E>(self, v: u8) -> Result<Number, E>
            where
                E: serde::de::Error,
            {
                Ok(Number::UInt(v as u128))
            }

            fn visit_u16<E>(self, v: u16) -> Result<Number, E>
            where
                E: serde::de::Error,
            {
                Ok(Number::UInt(v as u128))
            }

            fn visit_u32<E>(self, v: u32) -> Result<Number, E>
            where
                E: serde::de::Error,
            {
                Ok(Number::UInt(v as u128))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Number, E>
            where
                E: serde::de::Error,
            {
                Ok(Number::UInt(v as u128))
            }

            fn visit_u128<E>(self, v: u128) -> Result<Number, E>
            where
                E: serde::de::Error,
            {
                Ok(Number::UInt(v))
            }

            fn visit_f32<E>(self, v: f32) -> Result<Number, E>
            where
                E: serde::de::Error,
            {
                Ok(Number::Float(v as f64))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Number, E>
            where
                E: serde::de::Error,
            {
                Ok(Number::Float(v))
            }
        }

        deserializer.deserialize_any(NumberVisitor)
    }
}

#[cfg(feature = "serde")]
impl<'a> serde::Deserialize<'a> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        struct DateVisitor;

        impl<'a> serde::de::Visitor<'a> for DateVisitor {
            type Value = Date;

            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str("a date")
            }

            fn visit_str<E>(self, v: &str) -> Result<Date, E>
            where
                E: serde::de::Error,
            {
                let Ok(date) = v.parse() else {
                    struct Msg<'a>(&'a str);

                    impl<'a> Display for Msg<'a> {
                        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                            write!(f, "invalid date string, got {:?}", self.0)
                        }
                    }

                    return Err(E::custom(Msg(v)));
                };
                Ok(date)
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Date, E>
            where
                E: serde::de::Error,
            {
                struct Msg<'a>(&'a [u8]);

                impl<'a> Display for Msg<'a> {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        if let Ok(s) = core::str::from_utf8(self.0) {
                            write!(f, "invalid date string, got {:?}", s)
                        } else {
                            write!(f, "invalid date string, got {:?}", self.0)
                        }
                    }
                }

                if let Some((date, rem)) = parse_date(v) {
                    if rem.is_empty() {
                        Ok(date)
                    } else {
                        Err(E::custom(Msg(v)))
                    }
                } else {
                    Err(E::custom(Msg(v)))
                }
            }

            fn visit_map<A>(self, mut map: A) -> Result<Date, A::Error>
            where
                A: serde::de::MapAccess<'a>,
            {
                enum Field {
                    Year,
                    Month,
                    Day,
                }

                impl<'a> serde::Deserialize<'a> for Field {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'a>,
                    {
                        struct FieldVisitor;

                        impl<'a> serde::de::Visitor<'a> for FieldVisitor {
                            type Value = Field;

                            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                                f.write_str("'year', 'month', or 'day'")
                            }

                            fn visit_str<E>(self, v: &str) -> Result<Field, E>
                            where
                                E: serde::de::Error,
                            {
                                match v {
                                    "year" => Ok(Field::Year),
                                    "month" => Ok(Field::Month),
                                    "day" => Ok(Field::Day),
                                    _ => Err(E::unknown_field(v, &["year", "month", "day"])),
                                }
                            }
                        }

                        deserializer.deserialize_identifier(FieldVisitor)
                    }
                }

                let mut year: Option<u16> = None;
                let mut month: Option<u8> = None;
                let mut day: Option<u8> = None;

                while let Some(field) = map.next_key()? {
                    match field {
                        Field::Year => {
                            if year.is_none() {
                                year = Some(map.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("year"));
                            }
                        }
                        Field::Month => {
                            if month.is_none() {
                                month = Some(map.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("month"));
                            }
                        }
                        Field::Day => {
                            if day.is_none() {
                                day = Some(map.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("day"));
                            }
                        }
                    }
                }

                let Some(year) = year else {
                    return Err(serde::de::Error::missing_field("year"));
                };
                let Some(month) = month else {
                    return Err(serde::de::Error::missing_field("month"));
                };
                let Some(day) = day else {
                    return Err(serde::de::Error::missing_field("day"));
                };

                Ok(Date { year, month, day })
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Date, A::Error>
            where
                A: serde::de::SeqAccess<'a>,
            {
                let Some(year) = seq.next_element()? else {
                    return Err(serde::de::Error::invalid_length(0, &self));
                };
                let Some(month) = seq.next_element()? else {
                    return Err(serde::de::Error::invalid_length(1, &self));
                };
                let Some(day) = seq.next_element()? else {
                    return Err(serde::de::Error::invalid_length(2, &self));
                };
                let None = seq.next_element::<u8>()? else {
                    return Err(serde::de::Error::invalid_length(4, &self));
                };
                Ok(Date { year, month, day })
            }
        }

        deserializer.deserialize_any(DateVisitor)
    }
}

#[cfg(feature = "serde")]
impl<'a> serde::Deserialize<'a> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        struct TimeVisitor;

        impl<'a> serde::de::Visitor<'a> for TimeVisitor {
            type Value = Time;

            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str("a time")
            }

            fn visit_str<E>(self, v: &str) -> Result<Time, E>
            where
                E: serde::de::Error,
            {
                let Ok(time) = v.parse() else {
                    struct Msg<'a>(&'a str);

                    impl<'a> Display for Msg<'a> {
                        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                            write!(f, "invalid time string, got {:?}", self.0)
                        }
                    }

                    return Err(E::custom(Msg(v)));
                };
                Ok(time)
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Time, E>
            where
                E: serde::de::Error,
            {
                struct Msg<'a>(&'a [u8]);

                impl<'a> Display for Msg<'a> {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        if let Ok(s) = core::str::from_utf8(self.0) {
                            write!(f, "invalid time string, got {:?}", s)
                        } else {
                            write!(f, "invalid time string, got {:?}", self.0)
                        }
                    }
                }

                if let Some((time, rem)) = parse_time(v) {
                    if rem.is_empty() {
                        Ok(time)
                    } else {
                        Err(E::custom(Msg(v)))
                    }
                } else {
                    Err(E::custom(Msg(v)))
                }
            }

            fn visit_map<A>(self, mut map: A) -> Result<Time, A::Error>
            where
                A: serde::de::MapAccess<'a>,
            {
                enum Field {
                    Hour,
                    Minute,
                    Second,
                    Nanosecond,
                }

                impl<'a> serde::Deserialize<'a> for Field {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'a>,
                    {
                        struct FieldVisitor;

                        impl<'a> serde::de::Visitor<'a> for FieldVisitor {
                            type Value = Field;

                            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                                f.write_str("'hour', 'minute', 'second', or 'nanosecond'")
                            }

                            fn visit_str<E>(self, v: &str) -> Result<Field, E>
                            where
                                E: serde::de::Error,
                            {
                                match v {
                                    "hour" => Ok(Field::Hour),
                                    "minute" => Ok(Field::Minute),
                                    "second" => Ok(Field::Second),
                                    "nanosecond" => Ok(Field::Nanosecond),
                                    _ => Err(E::unknown_field(
                                        v,
                                        &["hour", "minute", "second", "nanosecond"],
                                    )),
                                }
                            }
                        }

                        deserializer.deserialize_identifier(FieldVisitor)
                    }
                }

                let mut hour: Option<u8> = None;
                let mut minute: Option<u8> = None;
                let mut second: Option<u8> = None;
                let mut nanosecond: Option<u32> = None;

                while let Some(field) = map.next_key()? {
                    match field {
                        Field::Hour => {
                            if hour.is_none() {
                                hour = Some(map.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("hour"));
                            }
                        }
                        Field::Minute => {
                            if minute.is_none() {
                                minute = Some(map.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("minute"));
                            }
                        }
                        Field::Second => {
                            if second.is_none() {
                                second = Some(map.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("second"));
                            }
                        }
                        Field::Nanosecond => {
                            if nanosecond.is_none() {
                                nanosecond = Some(map.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("nanosecond"));
                            }
                        }
                    }
                }

                let Some(hour) = hour else {
                    return Err(serde::de::Error::missing_field("hour"));
                };
                let Some(minute) = minute else {
                    return Err(serde::de::Error::missing_field("minute"));
                };
                let Some(second) = second else {
                    return Err(serde::de::Error::missing_field("second"));
                };
                let nanosecond = nanosecond.unwrap_or(0);

                Ok(Time {
                    hour,
                    minute,
                    second,
                    nanosecond,
                })
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Time, A::Error>
            where
                A: serde::de::SeqAccess<'a>,
            {
                let Some(hour) = seq.next_element()? else {
                    return Err(serde::de::Error::invalid_length(0, &self));
                };
                let Some(minute) = seq.next_element()? else {
                    return Err(serde::de::Error::invalid_length(1, &self));
                };
                let Some(second) = seq.next_element()? else {
                    return Err(serde::de::Error::invalid_length(2, &self));
                };
                let nanosecond = if let Some(nanosecond) = seq.next_element()? {
                    let None = seq.next_element::<u8>()? else {
                        return Err(serde::de::Error::invalid_length(5, &self));
                    };
                    nanosecond
                } else {
                    0
                };
                Ok(Time {
                    hour,
                    minute,
                    second,
                    nanosecond,
                })
            }
        }

        deserializer.deserialize_any(TimeVisitor)
    }
}

#[cfg(feature = "serde")]
impl<'a> serde::Deserialize<'a> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        struct DateTimeVisitor;

        impl<'a> serde::de::Visitor<'a> for DateTimeVisitor {
            type Value = DateTime;

            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str("a date and time")
            }

            fn visit_str<E>(self, v: &str) -> Result<DateTime, E>
            where
                E: serde::de::Error,
            {
                let Ok(dt) = v.parse() else {
                    struct Msg<'a>(&'a str);

                    impl<'a> Display for Msg<'a> {
                        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                            write!(f, "invalid date-time string, got {:?}", self.0)
                        }
                    }

                    return Err(E::custom(Msg(v)));
                };
                Ok(dt)
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<DateTime, E>
            where
                E: serde::de::Error,
            {
                struct Msg<'a>(&'a [u8]);

                impl<'a> Display for Msg<'a> {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        if let Ok(s) = core::str::from_utf8(self.0) {
                            write!(f, "invalid date-time string, got {:?}", s)
                        } else {
                            write!(f, "invalid date-time string, got {:?}", self.0)
                        }
                    }
                }

                if let Some((dt, rem)) = parse_datetime(v) {
                    if rem.is_empty() {
                        Ok(dt)
                    } else {
                        Err(E::custom(Msg(v)))
                    }
                } else {
                    Err(E::custom(Msg(v)))
                }
            }

            fn visit_map<A>(self, mut map: A) -> Result<DateTime, A::Error>
            where
                A: serde::de::MapAccess<'a>,
            {
                enum Field {
                    Date,
                    Time,
                    Offset,
                }

                impl<'a> serde::Deserialize<'a> for Field {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'a>,
                    {
                        struct FieldVisitor;

                        impl<'a> serde::de::Visitor<'a> for FieldVisitor {
                            type Value = Field;

                            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                                f.write_str("'date', 'time', or 'offset'")
                            }

                            fn visit_str<E>(self, v: &str) -> Result<Field, E>
                            where
                                E: serde::de::Error,
                            {
                                match v {
                                    "date" => Ok(Field::Date),
                                    "time" => Ok(Field::Time),
                                    "offset" => Ok(Field::Offset),
                                    _ => Err(E::unknown_field(v, &["date", "time", "offset"])),
                                }
                            }
                        }

                        deserializer.deserialize_identifier(FieldVisitor)
                    }
                }

                let mut date: Option<Date> = None;
                let mut time: Option<Time> = None;
                let mut offset: Option<Option<i16>> = None;

                while let Some(field) = map.next_key()? {
                    match field {
                        Field::Date => {
                            if date.is_none() {
                                date = Some(map.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("date"));
                            }
                        }
                        Field::Time => {
                            if time.is_none() {
                                time = Some(map.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                        }
                        Field::Offset => {
                            if offset.is_none() {
                                offset = Some(map.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                        }
                    }
                }

                let Some(date) = date else {
                    return Err(serde::de::Error::missing_field("date"));
                };
                let Some(time) = time else {
                    return Err(serde::de::Error::missing_field("time"));
                };
                let offset = offset.flatten();

                Ok(DateTime { date, time, offset })
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<DateTime, A::Error>
            where
                A: serde::de::SeqAccess<'a>,
            {
                let Some(date) = seq.next_element()? else {
                    return Err(serde::de::Error::invalid_length(0, &self));
                };
                let Some(time) = seq.next_element()? else {
                    return Err(serde::de::Error::invalid_length(1, &self));
                };
                let Some(offset) = seq.next_element()? else {
                    return Err(serde::de::Error::invalid_length(2, &self));
                };
                Ok(DateTime { date, time, offset })
            }
        }

        deserializer.deserialize_any(DateTimeVisitor)
    }
}
