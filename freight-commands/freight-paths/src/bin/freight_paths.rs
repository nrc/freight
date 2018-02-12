extern crate freight_paths;
extern crate data;
extern crate serde_json;

use freight_paths::{freight_paths, Error, PathArgs};
use std::path::Path;
use std::process;

fn main() {
    if let Err(_) = run() {
        handle_err();
    }
}

fn run() -> Result<(), Error> {
    let args = PathArgs::from_env();
    let paths = freight_paths(args)?;
    data::write_metadata(Path::new("paths.json"), &serde_json::to_string(&paths)?)?;
    Ok(())
}

fn handle_err() {
    eprintln!("An error occurred");
    process::exit(1);
}
