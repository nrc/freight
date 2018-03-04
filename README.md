# Freight

A Cargo re-implementation using a Git-like plumbing/porcelain architecture.


## Building and running

Use `cargo build` to build and `cargo run -p freight` to run the main binary.


### Arguments and env vars

`FREIGHT_DIR` specifies the directory to leave freight metadata files for the
current project. The default is `$PWD/.freight`.


## Design

Every Cargo command is decomposed into several Freight commands. Each Freight
command should be atomic. Cargo clients can then use Freight commands in order
to have much more fine-grained control of the build process without having to
have their own implementations of the functionality.

The architecture is inspired by Git's plumbing and porcelain approach: Freight
commands are the plumbing and Cargo commands are the porcelain. Although the
goal is for Cargo commands to just be sequences of calls to Freight, for the
sake of efficiency they should not be scripts, but use the API versions of the
freight commands.


## Principles

Each Freight command can be used either from the command line or as an API
function. The two should work identically.

Any intermediate data structures should be representable either on disk or in
memory. Clients can treat these structures as consumable metadata and can
reasonably expect to generate or modify them. They'll be stable, eventually.


## Repo layout

* cargo

Cargo commands re-implemented using freight commands (and the cargo binary).

* freight

The Freight binary

* freight-commands

Each directory contains a Freight command, e.g., freight-tidy contains the
`freight tidy` command for tidying up at the end of a cargo command.

* freight-data

Data structures passed between Freight commands


## Planning

This section is mostly speculative and sketchy.

use cases:

    * build systems (Facebook) [RFC](https://github.com/rust-lang/rfcs/pull/2136/files)
    * linux distros?
    * internet-free builds, very custom dep repositories
    * build deps only
    * RLS, IDEs (and when used with the above custom build systems)

cross-cutting concerns:
    * configuration, error handling, output handling, know about side effects (docs, save-analysis), VCS
    * verbose
    * frozen, locked

'global' options
    * output: `--verbose`, `--quiet`, `--color`, `--message-format`
    * `--frozen`, `--locked`
    * `-Z` 

### binaries

`cargo`

    * run cargo sub-commands
    * help page, version, etc.
    * aliasing of commands (from config)

`freight`

    * runs freight commands


### cargo build

discover files - config file, Cargo.toml

    * `freight paths`
    * --manifest-path

configuration <- CLI args, config files, env vars

    * `freight configure`
    * find cwd, home directory

profile selection (basically collates arguments for rustc, etc)

    * `freight profile`
    * --lib, --bin, --examples, --test(s), --bench(es)
    * --release
    * --target
    * features
    * Cargo.toml (profiles)

workspace discovery

    * `freight workspace`
    * targets from Cargo.toml + collect metadata
    * --package, --all, --exclude
    * could be re-used by cargo fmt

compute dep graph

    * `freight graph`
    * only the inter-crate edges
    * includes downloading dep metadata
    * propagate features to deps
    * input is either Cargo.toml and downloaded dep metadata or Cargo.lock + handle update, including per-package

make a build plan

    * `freight plan`
    * includes steps within a crate, including build scripts (build/run)
    * resolves the dep graph

execute build plan

    * `freight execute`
    * could include downloading dep source code
        - take into account patch/replace
    * orchestration vs actually executing rustc (distinction is important for RLS)
        - each step of plan -> rustc call (or rustdoc, etc)
            * build + run build scripts
        - do we need to actually run a step (i.e., has anything changed, freshness, fingerprinting)
    * parallelize jobs
        - `--jobs`

tidy up

    * `freight tidy`

what about?
    * build script - deps, build, run
    * 'temp' files for re-building (fingerprints)


#### helper commands

How do we easily replace a command? Do we need a level of indirection? Or does the replacer also need to replace the calling command? We actually have the same question with the top-level commands, but relative to Cargo commands.

* exec-rustc
* exec-rustdoc
* exec-script - run a build script
* fetch - download a dep
* stat - get dep metadata
* dep? - provide a dep source, from path or cache or server
* fresh - whether a package needs to be rebuilt
* read Cargo.toml
* read Cargo.lock
* write Cargo.lock
* write metadata file


#### decomposition from RFC:

| Dependency resolution | Lock file                                                                 | Custom registries, mirrors, offline/local, native deps, ... |
| Build configuration   | Cargo settings per crate in graph                                         | Profiles                                                    |
| Build lowering        | A build plan: a series of steps that must be run in sequence, including rustc and binary invocations | Build scripts, plugins           |
| Build execution       | Compiled artifacts                                                        | Caching                                                     |


#### help

```
Compile a local package and all of its dependencies

Usage:
    cargo build [options]

Options:
    -h, --help                   Print this message
    -p SPEC, --package SPEC ...  Package to build
    --all                        Build all packages in the workspace
    --exclude SPEC ...           Exclude packages from the build
    -j N, --jobs N               Number of parallel jobs, defaults to # of CPUs
    --lib                        Build only this package's library
    --bin NAME                   Build only the specified binary
    --bins                       Build all binaries
    --example NAME               Build only the specified example
    --examples                   Build all examples
    --test NAME                  Build only the specified test target
    --tests                      Build all tests
    --bench NAME                 Build only the specified bench target
    --benches                    Build all benches
    --all-targets                Build all targets (lib and bin targets by default)
    --release                    Build artifacts in release mode, with optimizations
    --features FEATURES          Space-separated list of features to also build
    --all-features               Build all available features
    --no-default-features        Do not build the `default` feature
    --target TRIPLE              Build for the target triple
    --manifest-path PATH         Path to the manifest to compile
    -v, --verbose ...            Use verbose output (-vv very verbose/build.rs output)
    -q, --quiet                  No output printed to stdout
    --color WHEN                 Coloring: auto, always, never
    --message-format FMT         Error format: human, json [default: human]
    --frozen                     Require Cargo.lock and cache are up to date
    --locked                     Require Cargo.lock is up to date
    -Z FLAG ...                  Unstable (nightly-only) flags to Cargo

If the --package argument is given, then SPEC is a package id specification
which indicates which package should be built. If it is not given, then the
current package is built. For more information on SPEC and its format, see the
`cargo help pkgid` command.

All packages in the workspace are built if the `--all` flag is supplied. The
`--all` flag is automatically assumed for a virtual manifest.
Note that `--exclude` has to be specified in conjunction with the `--all` flag.

Compilation can be configured via the use of profiles which are configured in
the manifest. The default profile for this command is `dev`, but passing
the --release flag will use the `release` profile instead.
```


### cargo update

* Cargo.toml -> Cargo.lock (force)


### cargo new

* check for exists already
* create dirs + files
* init VCS

