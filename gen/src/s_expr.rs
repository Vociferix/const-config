use crate::Value;
use std::io::{Read, Write};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    SExpr(#[from] serde_lexpr::error::Error),
}

type Result<T> = std::result::Result<T, Error>;

pub fn generate<I, O>(input: I, output: O) -> Result<()>
where
    I: Read,
    O: Write,
{
    crate::generate(serde_lexpr::from_reader(input)?, output)?;
    Ok(())
}
