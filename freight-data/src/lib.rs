#[macro_use]
extern crate serde_derive;

use std::env;
use std::fs::{DirBuilder, File};
use std::io::Error as IoError;
use std::io::{Write, Read};
use std::path::{Path, PathBuf};

const VERSION: &'static str = "0.1.0";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FilePaths {
    pub version: String,
    pub root_toml: String,
    pub workspace_root: String,
    pub target_dir: String,
    pub cargo_home: String,
    // These config files are in priority order with the highest priority first.
    pub config: Vec<String>,
}

impl FilePaths {
    pub fn new() -> FilePaths {
        FilePaths {
            version: VERSION.to_owned(),
            root_toml: String::new(),
            workspace_root: String::new(),
            target_dir: String::new(),
            cargo_home: String::new(),
            config: Vec::new(),
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    // pub config: HashMap<String, CargoValue>,
    pub cwd: PathBuf,
    pub workspace_root: PathBuf,
    pub root_toml: PathBuf,
    pub target_dir: PathBuf,
    pub cargo_home: PathBuf,
}

impl Config {
    pub fn new(
        cwd: PathBuf,
        file_paths: FilePaths,
    ) -> Config {
        Config {
            version: VERSION.to_owned(),
            // config: HashMap::new(),
            cwd,
            workspace_root: Path::new(&file_paths.workspace_root).to_owned(),
            root_toml: Path::new(&file_paths.root_toml).to_owned(),
            target_dir: Path::new(&file_paths.target_dir).to_owned(),
            cargo_home: Path::new(&file_paths.cargo_home).to_owned(),
        }
    }
}

// Will contain IO stuff too (shell/diagnostics, http)
pub struct Context {
    pub config: Config,
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

pub fn read_metadata(path: &Path) -> Result<String, IoError> {
    let mut abs_path = metadata_path()?;
    abs_path.push(path);
    let mut file = File::open(abs_path)?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
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

