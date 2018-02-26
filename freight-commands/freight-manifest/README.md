# freight manifest

Helper command to provide a manifest by reading Cargo.toml.

Caches the deserialised manifest in the freight context so repeated calls do not
re-read Cargo.toml.

## Input

A context (which contains a manifest path in the config), optionally an explicit
manifest path argument which overrides the path from the context. The binary
version (but not the API) will use the CWD if there is no manifest path argument.

## Output

A `data::manifest::Manifest` struct, or a file `manifest.json`.
