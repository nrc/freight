extern crate freight_paths;
extern crate freight_tidy;

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 || args[1] == "help" {
        println!("help");
    }

    if let Err(_) = run_command(&args[1]) {
        println!("Unhandled error");
    }
}

fn run_command(cmd: &str) -> Result<(), ()> {
    match cmd {
        "paths" => {
            // TODO sub-command args, not args?
            if let Err(_) = freight_paths::freight_paths(freight_paths::PathArgs::from_env()) {
                println!("Unhandled error");
            }
        }
        _ => println!("bad command"),
    }

    Ok(())
}
