# Chemical Abstracts Service (CAS) code

[![PGRX Build](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/pgrx-build-cas_codes.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/pgrx-build-cas_codes.yml)
[![Clippy](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-clippy-cas_codes.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-clippy-cas_codes.yml)
[![Test](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-cas_codes.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-cas_codes.yml)

Rust crate providing a structured representation of [CAS Registry Numbers (CAS RN)](https://en.wikipedia.org/wiki/CAS_Registry_Number) and related functionality.

## Usage

To use this crate, add the following to your `Cargo.toml`:

TODO: Add toml when published

## Example

Here is a simple example of how to use the `cas_codes` crate:

```rust
use cas_codes::CAS;

// Create a CAS object from a string, in this case
// the water CAS number.
let cas = CAS::try_from("7732-18-5").unwrap();

assert_eq!(cas.first(), 7732);
assert_eq!(cas.second(), 18);
assert_eq!(cas.check_digit(), 5);
assert_eq!(cas.digits().collect::<Vec<_>>(), vec![5, 8, 1, 2, 3, 7, 7]);
assert_eq!(cas.to_string(), "7732-18-5");
```

## Building the PGRX extension

This crate can be optionally built and used as a `PostgreSQL` extension. The extension is built using the [pgrx](https://github.com/pgcentralfoundation/pgrx) crate, which provides a framework for building `PostgreSQL` extensions in Rust.

To build this extension, just run the following docker build command:

```bash
USER_ID=$(id -u) GROUP_ID=$(id -g) docker compose up
```

Note that the `USER_ID` and `GROUP_ID` environment variables are used to set the user and group IDs inside the Docker container to match those of the host system. This is important for file permissions when mounting volumes and avoid writing out files with root permissions.
