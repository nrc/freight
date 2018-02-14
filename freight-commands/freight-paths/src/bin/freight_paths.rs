extern crate freight_paths;
extern crate data;
extern crate serde_json;
#[macro_use]
extern crate structopt;

use freight_paths::{freight_paths, Error, PathArgs};
use std::path::Path;
use std::process;
use structopt::StructOpt;

fn main() {
    if let Err(e) = run() {
        eprintln!("An error occurred: {:?}", e);
        process::exit(1);
    }
}
// TODO the usage uses the name of the binary from args, but that is wrong for subcommands
#[derive(StructOpt)]
#[structopt(name = "freight-paths", about = "Discover paths to cargo metadata files.")]
struct CliArgs {
    #[structopt(long = "manifest-path", help = "Path to the manifest to compile")]
    manifest_path: Option<String>,
}

fn run() -> Result<(), Error> {
    let cli_args = CliArgs::from_args();
    let args = PathArgs::from_env(cli_args.manifest_path);
    let paths = freight_paths(args)?;
    data::write_metadata(Path::new("paths.json"), &serde_json::to_string(&paths)?)?;
    Ok(())
}
