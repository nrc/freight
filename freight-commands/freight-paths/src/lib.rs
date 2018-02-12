extern crate data;
extern crate serde_json;
#[macro_use]
extern crate structopt;

use data::FilePaths;
use std::env;
use std::io::Error as IoError;
use structopt::StructOpt;

#[derive(Debug, Clone)]
pub struct PathArgs {
    pub manifest_path: Option<String>,
    pub cargo_home: Option<String>,
    pub cargo_target_dir: Option<String>,
}

impl PathArgs {
    pub fn from_env() -> PathArgs {
        let cli_args = CliArgs::from_args();
        let manifest_path = cli_args.manifest_path;
        let cargo_home = env::var("CARGO_HOME").ok();
        let cargo_target_dir = env::var("CARGO_TARGET_DIR").ok();

        PathArgs {
            manifest_path,
            cargo_home,
            cargo_target_dir,
        }
    }
}

// TODO the usage uses the name of the binary from args, but that is wrong for subcommands
#[derive(StructOpt)]
#[structopt(name = "freight-paths", about = "Discover paths to cargo metadata files.")]
pub struct CliArgs {
    #[structopt(long = "manifest-path", help = "Path to the manifest to compile")]
    pub manifest_path: Option<String>,
    // Decoy args so running via freight works.
    pub paths: Option<String>,
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
