use crate::Value;
use std::io::{Read, Write};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    FlexBuffers(#[from] flexbuffers::DeserializationError),
}

type Result<T> = std::result::Result<T, Error>;

pub fn generate<I, O>(mut input: I, output: O) -> Result<()>
where
    I: Read,
    O: Write,
{
    let mut buf = Vec::new();
    input.read_to_end(&mut buf)?;
    crate::generate(flexbuffers::from_slice(&buf)?, output)?;
    Ok(())
}
