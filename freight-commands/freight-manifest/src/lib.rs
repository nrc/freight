extern crate data;
#[macro_use]
extern crate failure;
extern crate serde_json;
extern crate toml;

use data::{Context, UserKey, UserKind};
use data::manifest::{Manifest, Metadata};

use std::collections::HashMap;
use std::str::FromStr;
use std::fs::File;
use std::io::{Error as IoError, Read};
use std::path::{Path, PathBuf};
use toml::value::Value;

#[derive(Debug, Clone, Fail)]
pub enum Error {
    #[fail(display = "No manifest found at {:?}", path)]
    MissingManifest { path: PathBuf },
    #[fail(display = "io error")]
    Io,
    #[fail(display = "TOML parsing error, {}", message)]
    Toml { message: String },
}

impl Error {
    fn toml(message: &str) -> Error {
        Error::Toml{ message: message.to_owned() }
    }
}

impl From<IoError> for Error {
    fn from(_: IoError) -> Error {
        Error::Io
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Error {
        Error::Io
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Error {
        Error::Toml { message: e.to_string() }
    }
}

// In the user-data of the context we store each manifest indexed by path via
// a lookup table, the lookup table is stored in the user-data at id = CACHE_ID.
const CACHE_ID: u32 = 0;

struct ManifestCache {
    cache: HashMap<PathBuf, u32>,
    next: u32,
}

impl ManifestCache {
    fn new() -> ManifestCache {
        ManifestCache {
            cache: HashMap::new(),
            next: 1,
        }
    }
}

fn user_key(id: u32) -> UserKey {
    UserKey::new(UserKind::Manifest, id)
}

fn get_from_cache(path: &Path, context: &Context) -> Option<Manifest> {
    let user_data = context.user_data.lock().unwrap();
    let cache = &user_data
        .get(&user_key(CACHE_ID))?
        .downcast_ref::<ManifestCache>()
        .expect("bad type, expected ManifestCache")
        .cache;
    let id = cache.get(path)?;
    Some(user_data
        .get(&user_key(*id))?
        .downcast_ref::<Manifest>()
        .expect("bad type, expected Manifest")
        .clone())
}

fn store_into_cache(path: &Path, manifest: Manifest, context: &Context) {
    let mut user_data = context.user_data.lock().unwrap();
    let id = {
        let cache = user_data
            .entry(user_key(CACHE_ID))
            .or_insert_with(|| Box::new(ManifestCache::new()))
            .downcast_mut::<ManifestCache>()
            .expect("bad type, expected ManifestCache");
        let id = cache.next;
        cache.next += 1;
        cache.cache.insert(path.to_owned(), id);
        id
    };
    user_data.insert(user_key(id), Box::new(manifest));
}

fn read_manifest(path: &Path) -> Result<Manifest, Error> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let value = Value::from_str(&buf)?;
    let top = value.as_table().ok_or_else(|| Error::toml("Expected table"))?;

    let mut result = Manifest::new();

    if let Some(package) = top.get("package") {
        let package = package.as_table().ok_or_else(|| Error::toml("Expected table"))?;
        result.metadata = Some(Metadata {
            name: package
                .get("name")
                .ok_or_else(|| Error::toml("Expected `name`"))?
                .as_str()
                .ok_or_else(|| Error::toml("`name` not a string"))?
                .to_owned(),
            version: package
                .get("version")
                .ok_or_else(|| Error::toml("Expected `version`"))?
                .as_str()
                .ok_or_else(|| Error::toml("`version` not a string"))?
                .to_owned(),
            authors: package
                .get("authors")
                .ok_or_else(|| Error::toml("Expected `authors`"))?
                .as_array()
                .ok_or_else(|| Error::toml("`authors` not an array"))?
                .iter()
                .map(|v| v
                    .as_str()
                    .ok_or_else(|| Error::toml("author not a string"))
                    .map(|s| s.to_owned()))
                .collect::<Result<Vec<String>, _>>()?,
        });
    }

    // TODO profiles

    Ok(result)
}

pub fn freight_manifest(manifest_path: Option<&Path>, context: &Context) -> Result<Manifest, Error> {
    // TODO find the actual toml file, don't use root
    let path = manifest_path.unwrap_or(&context.config.root_toml);
    if let Some(manifest) = get_from_cache(path, context) {
        return Ok(manifest);
    }

    let result = read_manifest(path);
    if let Ok(ref manifest) = result {
        store_into_cache(path, manifest.clone(), context)
    }

    result
}
