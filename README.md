# Freight

A Cargo re-implementation using a Git-like plumbing/porcelain architecture.


## Building and running

Use `cargo build` to build and `cargo run -p freight` to run the main binary.


## Design

Every Cargo command is decomposed into several Freight commands. Each Freight
command should be atomic. Cargo clients can then use Freight commands in order
to have much more fine-grained control of the build process without having to
have their own implementations of the functionality.


## Principles

Each Freight command can be used either from the command line or as an API
function. The two should work identically.

Any intermediate data structures should be representable either on disk or in
memory. Clients can treat these structures as consumable metadata and can
reasonably expect to generate or modify them.


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

