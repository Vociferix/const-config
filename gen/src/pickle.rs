use crate::Value;
use std::io::{Read, Write};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Pickle(#[from] serde_pickle::error::Error),
}

type Result<T> = std::result::Result<T, Error>;

pub fn generate<I, O>(input: I, output: O) -> Result<()>
where
    I: Read,
    O: Write,
{
    crate::generate(
        serde_pickle::from_reader(
            input,
            serde_pickle::DeOptions::new()
                .decode_strings()
                .keep_restore_state(),
        )?,
        output,
    )?;
    Ok(())
}
