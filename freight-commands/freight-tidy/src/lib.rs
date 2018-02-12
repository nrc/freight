extern crate data;

use std::fs::remove_dir_all;
use std::io::Error as IoError;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    Io,
}

impl From<IoError> for Error {
    fn from(_: IoError) -> Error {
        Error::Io
    }
}

pub fn freight_tidy() -> Result<(), Error> {
    let metadata_path = data::metadata_path()?;
    if !metadata_path.exists() {
        return Ok(());
    }

    remove_dir_all(&metadata_path)?;
    Ok(())
}