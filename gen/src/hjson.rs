use crate::Value;
use std::io::{Read, Write};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    HJson(#[from] deser_hjson::Error),
}

type Result<T> = std::result::Result<T, Error>;

pub fn generate<I, O>(input: I, output: O) -> Result<()>
where
    I: Read,
    O: Write,
{
    crate::generate(deser_hjson::from_reader(input)?, output)?;
    Ok(())
}
