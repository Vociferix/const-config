// We don't use Value's Deserialize implementation because
// we wouldn't be able to deserialize a Time or DateTime.
// Instead, we use `toml::from_str` to parse the TOML text
// and then convert to a `crate::Value` to ensure that
// Times and DateTimes are preserved.

use crate::{Date, DateTime, Time, Value};
use std::io::{self, Read, Write};
use thiserror::Error;
use toml::Value as TomlValue;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}

type Result<T> = std::result::Result<T, Error>;

pub fn generate<I, O>(mut input: I, output: O) -> Result<()>
where
    I: Read,
    O: Write,
{
    let mut toml_s = String::new();
    input.read_to_string(&mut toml_s)?;
    crate::generate(transform(&::toml::from_str(&toml_s)?), output)?;
    Ok(())
}

fn transform(toml: &TomlValue) -> Value {
    match toml {
        TomlValue::String(value) => Value::Str(value.clone()),
        TomlValue::Integer(value) => Value::Int(*value),
        TomlValue::Float(value) => Value::Float(*value),
        TomlValue::Boolean(value) => Value::Bool(*value),
        TomlValue::Datetime(value) => transform_datetime(value),
        TomlValue::Array(value) => {
            let mut array = Vec::with_capacity(value.len());
            for elem in value {
                array.push(transform(elem));
            }
            Value::Array(array)
        }
        TomlValue::Table(value) => {
            let mut obj = Vec::with_capacity(value.len());
            for (key, value) in value {
                obj.push((key.clone(), transform(value)));
            }
            Value::Object(obj)
        }
    }
}

fn transform_datetime(dt: &::toml::value::Datetime) -> Value {
    if let Some(date) = &dt.date {
        let date = Date {
            year: date.year,
            month: date.month,
            day: date.day,
        };
        if let Some(time) = &dt.time {
            let time = Time {
                hour: time.hour,
                minute: time.minute,
                second: time.second,
                nanosecond: time.nanosecond,
            };
            if let Some(offset) = &dt.offset {
                let offset = if let ::toml::value::Offset::Custom { minutes } = offset {
                    *minutes
                } else {
                    0i16
                };
                Value::DateTime(DateTime {
                    date,
                    time,
                    offset: Some(offset),
                })
            } else {
                Value::DateTime(DateTime {
                    date,
                    time,
                    offset: None,
                })
            }
        } else {
            Value::Date(date)
        }
    } else if let Some(time) = &dt.time {
        Value::Time(Time {
            hour: time.hour,
            minute: time.minute,
            second: time.second,
            nanosecond: time.nanosecond,
        })
    } else {
        Value::Time(Time {
            hour: 0,
            minute: 0,
            second: 0,
            nanosecond: 0,
        })
    }
}
