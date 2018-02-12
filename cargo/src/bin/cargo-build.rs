extern crate freight_paths;
#[macro_use]
extern crate structopt;

use freight_paths::{freight_paths, Error, PathArgs};
use std::process;
use structopt::StructOpt;


fn main() {
    if let Err(_) = run() {
        handle_err();
    }
}

#[derive(StructOpt)]
#[structopt(name = "cargo-build", about = "Compile a local package and all of its dependencies.")]
struct CliArgs {
    #[structopt(long = "manifest-path", help = "Path to the manifest to compile")]
    manifest_path: Option<String>,
}

// TODO need to use Failure::Error, rather than freight_paths::Error here.
fn run() -> Result<(), Error> {
    let cli_args = CliArgs::from_args();
    let args = PathArgs::from_env(cli_args.manifest_path);
    let _paths = freight_paths(args)?;
    Ok(())
}

fn handle_err() {
    eprintln!("An error occurred");
    process::exit(1);
}
