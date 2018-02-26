extern crate data;
extern crate freight_manifest;
extern crate serde_json;
#[macro_use]
extern crate structopt;

use data::Context;
use freight_manifest::{freight_manifest, Error};
use std::env;
use std::path::{Path, PathBuf};
use std::process;
use structopt::StructOpt;

fn main() {
    if let Err(e) = run() {
        handle_err(e);
    }
}

#[derive(StructOpt)]
#[structopt(name = "freight-manifest", about = "Read the Cargo manifest, Cargo.toml.")]
struct CliArgs {
    #[structopt(long = "manifest-path", help = "Path to the manifest")]
    manifest_path: Option<String>,
}

fn run() -> Result<(), Error> {
    let cli_args = CliArgs::from_args();
    let path = cli_args.manifest_path.map(|p| PathBuf::from(p)).unwrap_or_else(|| {
        let cwd = env::current_dir().expect("No cwd");
        cwd.join("Cargo.toml")
    });
    let manifest = freight_manifest(Some(&path), &Context::empty())?;
    data::write_metadata(Path::new("manifest.json"), &serde_json::to_string(&manifest)?)?;
    Ok(())
}

fn handle_err(e: Error) {
    eprintln!("An error occurred: {}", e);
    process::exit(1);
}
