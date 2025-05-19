# Elements

[![PGRX Build](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/pgrx-build-elements.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/pgrx-build-elements.yml)
[![Clippy](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-clippy-elements.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-clippy-elements.yml)
[![Test](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-elements.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-elements.yml)

Crate proving an enumeration and metadata for the elements of the periodic table.

## Compiling the PGRX extension

After having cloned the repository, you can compile the PGRX extension in the `./extension` directory by running in this directory:

```bash
USER_ID=$(id -u) GROUP_ID=$(id -g) docker compose up
```

Note that the `USER_ID` and `GROUP_ID` environment variables are used to set the user and group IDs inside the Docker container to match those of the host system. This is important for file permissions when mounting volumes and avoid writing out files with root permissions.
