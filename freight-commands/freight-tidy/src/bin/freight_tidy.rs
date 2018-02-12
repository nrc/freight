extern crate freight_tidy;

use freight_tidy::{freight_tidy, Error};
use std::process;

fn main() {
    if let Err(_) = run() {
        handle_err();
    }
}

fn run() -> Result<(), Error> {
    freight_tidy()
}

fn handle_err() {
    eprintln!("An error occurred");
    process::exit(1);
}
