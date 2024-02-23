# Contributing to the backend
This document describes how to contribute to the backend of the project.

## Diesel
Diesel is a ORM and query builder for Rust. It is designed to be safe and composable. It's supports Postgres, MySQL, and SQLite. In this project we will be using Postgres.

### Install Postgres
Before installing Diesel, you need to install Postgres. Note that without having installed Postgres, Diesel will not compile and fail
with a linking error:

```bash
linking with `cc` failed: exit code: 1
```

Depending on your OS, you can install it with the following commands:

#### MacOS
On MacOS, you can install Postgres with Homebrew:

`brew install postgresql`

### Install Diesel
You can install Diesel with the following command:

`cargo install diesel_cli --no-default-features --features postgres`

### Defining the position of the migrations folder
Diesel uses migrations to manage the database schema. To define the position of the migrations folder, create a file called `diesel.toml` in the root of the project and add the following content:

```toml
[migrations_directory]
dir = "migrations"
```

### Adding a table with Diesel
Diesel, as several other ORM, uses migrations to manage the database schema. To create a migration, run the following command:

`diesel migration generate create_{table_name}`

where `{table_name}` is the name of the table you want to create. This will create a new migration file in the `migrations` folder, which is at the position defined in the `diesel.toml` file.

### Setting the environment variables
Diesel uses environment variables to connect to the database. To set the environment variables, create a file called `.env` in the root of the project based on the `default_env` file.

### Running the migrations
To run the migrations, run the following command:

`diesel migration run`

## Starting cargo watch
Cargo watch is a tool that watches for changes in the project and automatically recompiles the project.

To install it, as per many other Rust tools, run the following command:

`cargo install cargo-watch`

To start cargo watch, run the following command:

`cargo watch -q -c -w src/ -x run`