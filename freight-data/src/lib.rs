#[macro_use]
extern crate serde_derive;

const VERSION: &'static str = "0.1.0";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FilePaths {
    version: String,
    root_toml: String,
    lock: String,
    // These config files are in priority order with the highest priority first.
    config: Vec<String>,
}

impl FilePaths {
    pub fn new() -> FilePaths {
        FilePaths {
            version: VERSION.to_owned(),
            root_toml: String::new(),
            lock: String::new(),
            config: Vec::new(),
        }
    }
}
