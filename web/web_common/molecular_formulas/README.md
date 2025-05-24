# Molecular formula

[![PGRX Build](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/pgrx-build-molecular_formulas.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/pgrx-build-molecular_formulas.yml)
[![Clippy](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-clippy-molecular_formulas.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-clippy-molecular_formulas.yml)
[![Test](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-molecular_formulas.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-molecular_formulas.yml)

Crate proving a parser, structs and utilities for molecular formulas.

## Compiling the PGRX extension

After having cloned the repository, you can compile the PGRX extension in the `./extension` directory by running in this directory:

```bash
USER_ID=$(id -u) GROUP_ID=$(id -g) docker compose up
```

Note that the `USER_ID` and `GROUP_ID` environment variables are used to set the user and group IDs inside the Docker container to match those of the host system. This is important for file permissions when mounting volumes and avoid writing out files with root permissions.
