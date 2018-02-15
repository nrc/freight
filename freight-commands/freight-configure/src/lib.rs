extern crate data;
#[macro_use]
extern crate failure;
extern crate serde_json;

use data::{Config, FilePaths};
use std::path::PathBuf;
use std::io::Error as IoError;


#[derive(Debug, Clone, Copy, Fail)]
pub enum Error {
    #[fail(display = "io error")]
    Io,
    #[fail(display = "unknown error")]
    Unknown,
}

impl From<IoError> for Error {
    fn from(_: IoError) -> Error {
        Error::Io
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Error {
        Error::Io
    }
}

pub fn freight_configure(
    cwd: PathBuf,
    file_paths: FilePaths,
) -> Result<Config, Error> {
    // FIXME should read in all the config files and push the config items into the result
    Ok(Config::new(cwd, file_paths))
}
