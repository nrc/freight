extern crate freight_configure;
extern crate data;
extern crate serde_json;
// #[macro_use]
// extern crate structopt;

use freight_configure::{freight_configure, Error};
use std::env;
use std::path::Path;
use std::process;
//use structopt::StructOpt;

fn main() {
    if let Err(e) = run() {
        eprintln!("An error occurred: {:?}", e);
        process::exit(1);
    }
}

// #[derive(StructOpt)]
// #[structopt(name = "freight-paths", about = "Discover paths to cargo metadata files.")]
// struct CliArgs {
//     #[structopt(long = "manifest-path", help = "Path to the manifest to compile")]
//     manifest_path: Option<String>,
// }

fn run() -> Result<(), Error> {
    let cwd = env::current_dir().expect("No cwd");
    let file_paths = data::read_metadata(&Path::new("paths.json")).expect("Missing paths.json");
    let file_paths = serde_json::from_str(&file_paths)?;
    let config = freight_configure(cwd, file_paths)?;
    data::write_metadata(Path::new("config.json"), &serde_json::to_string(&config)?)?;
    Ok(())
}
