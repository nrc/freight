#[macro_use]
extern crate serde_derive;

use std::env;
use std::fs::{DirBuilder, File};
use std::io::Error as IoError;
use std::io::Write;
use std::path::{Path, PathBuf};

const VERSION: &'static str = "0.1.0";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FilePaths {
    version: String,
    root_toml: String,
    lock: String,
    // These config files are in priority order with the highest priority first.
    config: Vec<String>,
}

impl FilePaths {
    pub fn new() -> FilePaths {
        FilePaths {
            version: VERSION.to_owned(),
            root_toml: String::new(),
            lock: String::new(),
            config: Vec::new(),
        }
    }
}

/// Write metadata to a file given by the relative path.
pub fn write_metadata(path: &Path, content: &str) -> Result<(), IoError> {
    let mut abs_path = metadata_path()?;
    abs_path.push(path);

    DirBuilder::new()
        .recursive(true)
        .create(abs_path.parent().expect("metadata path has no parent"))?;

    let mut file = File::create(abs_path)?;
    file.write_all(content.as_bytes())
}

/// Path to a directory for storing Freight's metadata files. 
pub fn metadata_path() -> Result<PathBuf, IoError> {
    match env::var("FREIGHT_DIR") {
        Ok(freight_dir) => Ok(PathBuf::from(freight_dir)),
        Err(_) => {
            let mut result = env::current_dir()?;
            result.push(".freight");
            Ok(result)
        }
    } 
}

