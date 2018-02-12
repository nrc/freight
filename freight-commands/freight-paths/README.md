# freight paths

Discovers and decides upon paths to metadata files (Cargo.toml, config files, etc).


## Input

* arguments
  - `--manifest-path`
* env vars
  - `CARGO_HOME`
  - `CARGO_TARGET_DIR`


## Output

A `data::FilePaths` struct, or a file `paths.json`.
