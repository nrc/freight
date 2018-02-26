//! A representation of the project's manifest, usually taken from Cargo.toml.

use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub version: String,
    pub metadata: Option<Metadata>,
    pub profiles: HashMap<String, Profile>,
}

impl Manifest {
    pub fn new() -> Manifest {
        Manifest {
            version: ::VERSION.to_owned(),
            metadata: None,
            profiles: HashMap::new(),
        }
    }
}

// TODO should we separate essential package data (name, version) from crates.io metadata?
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    pub debug: bool,
    pub panic: String,
}
