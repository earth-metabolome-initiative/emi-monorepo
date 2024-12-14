# Contributing

The validations crate defines data validation rules meant to be used across the three different levels of validations:

* Client-side validations, meant to be used in the frontend to provide immediate feedback to the user.
* Server-side validations, meant to be used in the backend to ensure data integrity and avoid unnecessary database queries - never trust the requests coming from the client!
* Database-level validations, meant to be used in the database to ensure data consistency and avoid data corruption - never trust the competence of the backend developer!

## Getting started

We try to use recent versions of the PostgreSQL database, and in order to compile the correct version of the `*.so` files, we need to have the correct `pg_config` command available in the system. At the time of writing, we use `PostgreSQL 17`. You can verify the version of postgres you have installed by running:

```bash
pg_config --version
```

In our case, it should output something like `PostgreSQL 17.2 (Ubuntu 17.2-1.pgdg22.04+1)`. If you don't have the `pg_config` command available or it has an earlier version, you can install it by running:

```bash
# Add PostgreSQL's official repository
sudo sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'
wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -

# Update and install PostgreSQL 17 dev tools
sudo apt update
sudo apt install postgresql-17 postgresql-server-dev-17
```

The validations crate is a library that can be used in any Rust project. In order to support the database-level validations, this crate uses the [`pgrx`](https://github.com/pgcentralfoundation/pgrx) crate. To install it, you need to run the following command:

```bash
cargo install --locked cargo-pgrx
```

Once you have installed the `pgrx` crate, you will have to run a first initialization command to create the necessary files and directories:

```bash
cargo pgrx init
```

Both commands may take a while to run, and may fail if some of the additional system dependencies are not installed.

Once all of the above is done, to compile a new version of the `pgrx` extension, you can run:

```bash
cargo pgrx package
```

This command will compile (in release mode) the extension for the current version of your `pg_config`, so **make sure you have the correct version installed**. The command will output the path to the generated `*.so`, `*.sql`, and `*.control` files. These documents should be found in the `target/release/` directory of this monorepo. For instance, at the time of writing the ouput path is `emi-monorepo/target/release/validations-pg17/usr/share/postgresql/17/extension/validations--0.0.0.sql`.
