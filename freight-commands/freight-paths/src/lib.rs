extern crate data;
extern crate serde_json;

use data::FilePaths;
use std::env;
use std::io::Error as IoError;

#[derive(Debug, Clone)]
pub struct PathArgs {
    pub manifest_path: Option<String>,
    pub cargo_home: Option<String>,
    pub cargo_target_dir: Option<String>,
}

impl PathArgs {
    pub fn from_env(manifest_path: Option<String>) -> PathArgs {
        let cargo_home = env::var("CARGO_HOME").ok();
        let cargo_target_dir = env::var("CARGO_TARGET_DIR").ok();

        PathArgs {
            manifest_path,
            cargo_home,
            cargo_target_dir,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    Io,
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

pub fn freight_paths(args: PathArgs) -> Result<FilePaths, Error> {
    let mut result = FilePaths::new();
    // TODO
    Ok(result)
}
