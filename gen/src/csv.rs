use crate::Value;
use std::io::{Read, Write};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Csv(#[from] csv::Error),
}

type Result<T> = std::result::Result<T, Error>;

pub fn generate<I, O>(input: I, output: O) -> Result<()>
where
    I: Read,
    O: Write,
{
    let mut rdr = csv::Reader::from_reader(input);
    let mut rows = Vec::new();
    for row in rdr.deserialize() {
        rows.push(row?);
    }
    crate::generate(Value::Array(rows), output)?;
    Ok(())
}
