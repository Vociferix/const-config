use crate::Value;
use json::JsonValue;
use std::io::{self, Read, Write};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    Json(#[from] json::Error),
}

type Result<T> = std::result::Result<T, Error>;

pub fn generate<I, O>(mut input: I, output: O) -> Result<()>
where
    I: Read,
    O: Write,
{
    let mut json_s = String::new();
    input.read_to_string(&mut json_s)?;
    crate::generate(transform(&json::parse(&json_s)?), output)?;
    Ok(())
}

fn transform(json: &JsonValue) -> Value {
    match json {
        JsonValue::Null => Value::Null,
        JsonValue::Short(value) => Value::Str((*value).into()),
        JsonValue::String(value) => Value::Str(value.clone()),
        JsonValue::Number(value) => {
            if let Ok(value) = u64::try_from(*value) {
                Value::UInt(value)
            } else if let Ok(value) = i64::try_from(*value) {
                Value::Int(value)
            } else {
                Value::Float((*value).into())
            }
        }
        JsonValue::Boolean(value) => Value::Bool(*value),
        JsonValue::Object(value) => Value::Object(
            value
                .iter()
                .map(|entry| (entry.0.into(), transform(entry.1)))
                .collect(),
        ),
        JsonValue::Array(value) => Value::Array(value.iter().map(transform).collect()),
    }
}
