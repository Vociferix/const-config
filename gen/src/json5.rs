use crate::Value;
use std::io::{Read, Write};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Json5(#[from] json5::Error),
}

type Result<T> = std::result::Result<T, Error>;

pub fn generate<I, O>(mut input: I, output: O) -> Result<()>
where
    I: Read,
    O: Write,
{
    let mut s = String::new();
    input.read_to_string(&mut s)?;
    crate::generate(json5::from_str(&s)?, output)?;
    Ok(())
}
