extern crate data;
#[macro_use]
extern crate failure;
extern crate serde_json;

use data::FilePaths;
use std::env;
use std::io::Error as IoError;

#[derive(Debug, Clone)]
pub struct PathArgs {
    pub manifest_path: Option<String>,
    pub cargo_home: Option<String>,
    pub cargo_target_dir: Option<String>,
    pub cwd: Option<String>,
}

impl PathArgs {
    pub fn from_env(manifest_path: Option<String>) -> PathArgs {
        let cargo_home = env::var("CARGO_HOME").ok();
        let cargo_target_dir = env::var("CARGO_TARGET_DIR").ok();
        let cwd = env::current_dir().ok().and_then(|cwd| cwd.to_str().map(|cwd| cwd.to_owned()));

        PathArgs {
            manifest_path,
            cargo_home,
            cargo_target_dir,
            cwd,
        }
    }
}

#[derive(Debug, Clone, Copy, Fail)]
pub enum Error {
    #[fail(display = "io error")]
    Io,
    #[fail(display = "cannot determine location of Cargo.toml")]
    Toml,
    #[fail(display = "bad cwd")]
    Cwd,
    #[fail(display = "could not find home directory")]
    Home,
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

pub fn freight_paths(args: PathArgs) -> Result<FilePaths, Error> {
    let cwd = match args.cwd {
        Some(cwd) => cwd,
        None => return Err(Error::Cwd),
    };

    let mut result = FilePaths::new();
    if let Some(manifest_path) = args.manifest_path {
        result.root_toml = manifest_path;
    } else {
        result.root_toml = cwd.clone();
        // FIXME should use Path::join rather than strings
        result.root_toml.push_str("/Cargo.toml");
    }

    {
        let toml_path = Path::new(&result.root_toml);
        if !toml_path.is_file() {
            return Err(Error::Toml);
        }
        match toml_path.parent() {
            Some(root) => {
                result.workspace_root = root.to_str().expect("Could not stringify workspace path").to_owned();
            }
            None => return Err(Error::Toml),
        }
    }

    match args.cargo_home {
        Some(cargo_home) => {
            result.cargo_home = cargo_home;
        }
        None => {
            result.cargo_home = match homedir(Path::new(&cwd)) {
                Some(home) => home.to_str().expect("Could not stringify cargo home").to_owned(),
                None => return Err(Error::Home),
            }
        }
    }

    match args.cargo_target_dir {
        Some(cargo_target_dir) => {
            result.target_dir = cargo_target_dir;
        }
        None => {
            let mut default_target_dir = result.workspace_root.clone();
            // FIXME use Paths
            default_target_dir.push_str("/target");
            result.target_dir = default_target_dir;
        }
    }

    walk_tree(Path::new(&cwd), |path| {
        result.config.push(path.to_str().expect("Could not stringify path").to_owned());
        Ok(())
    })?;

    Ok(result)
}

extern crate home;

fn homedir(cwd: &Path) -> Option<PathBuf> {
    ::home::cargo_home_with_cwd(cwd).ok()
}


use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::fs;

// From https://github.com/rust-lang/cargo/blob/master/src/cargo/util/config.rs
fn walk_tree<F>(pwd: &Path, mut walk: F) -> Result<(), Error>
    where F: FnMut(&Path) -> Result<(), Error>
{

    let mut stash: HashSet<PathBuf> = HashSet::new();

    for current in ancestors(pwd) {
        let possible = current.join(".cargo").join("config");
        if fs::metadata(&possible).is_ok() {
            walk(&possible)?;
            stash.insert(possible);
        }
    }

    // Once we're done, also be sure to walk the home directory even if it's not
    // in our history to be sure we pick up that standard location for
    // information.
    let home = homedir(pwd).ok_or_else(|| {
        Error::Home
    })?;
    let config = home.join("config");
    if !stash.contains(&config) && fs::metadata(&config).is_ok() {
        walk(&config)?;
    }

    Ok(())
}

// From https://github.com/rust-lang/cargo/blob/master/src/cargo/util/paths.rs
pub fn ancestors(path: &Path) -> PathAncestors {
    PathAncestors::new(path)
}

pub struct PathAncestors<'a> {
    current: Option<&'a Path>,
    stop_at: Option<PathBuf>
}

impl<'a> PathAncestors<'a> {
    fn new(path: &Path) -> PathAncestors {
        PathAncestors {
            current: Some(path),
            //HACK: avoid reading `~/.cargo/config` when testing Cargo itself.
            stop_at: env::var("__CARGO_TEST_ROOT").ok().map(PathBuf::from),
        }
    }
}

impl<'a> Iterator for PathAncestors<'a> {
    type Item = &'a Path;

    fn next(&mut self) -> Option<&'a Path> {
        if let Some(path) = self.current {
            self.current = path.parent();

            if let Some(ref stop_at) = self.stop_at {
                if path == stop_at {
                    self.current = None;
                }
            }

            Some(path)
        } else {
            None
        }
    }
}

