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

```bash
brew install postgresql
```

#### Ubuntu

On Ubuntu, you can install Postgres with the following commands:

```bash
sudo apt-get update && sudo apt-get install -y libpq-dev
```

### Install Diesel

You can install Diesel with the following command:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

### Install Python
First, check whether you already have python installed by running the following command:

```bash
python --version
```

Alternatively, you may have Python 3 installed without an alias. In this case, you can check whether you have Python 3 installed by running the following command:

```bash
python3 --version
```

If you do not have Python installed, we suggest you to install it using some package manager
that also takes care about the environments such as conda. You can install conda by running
the instructions [detailed here](https://docs.anaconda.com/free/miniconda/).

After having installed conda, you can create a new environment by running the following command:

```bash
conda create -n emi python=3.9
```

To activate the environment, run the following command:

```bash
conda activate emi
```

### Install the python dependencies
To handle some of the Diesel migrations, we have created some Python utilities.
The Python dependencies are listed in the `requirements.txt` file, which is in the `web/backend` folder.

To install the Python dependencies, run the following command:

```bash
pip install -r requirements.txt
```

### Defining the position of the migrations folder
Diesel uses migrations to manage the database schema. To define the position of the migrations folder, create a file called `diesel.toml` in the root of the project and add the following content:

```toml
[migrations_directory]
dir = "migrations"
```

### Adding a table with Diesel
Diesel, as several other ORM, uses migrations to manage the database schema. To create a migration, run the following command:

```bash
diesel migration generate create_{table_name}
```

where `{table_name}` is the name of the table you want to create. This will create a new migration file in the `migrations` folder, which is at the position defined in the `diesel.toml` file.

### Setting the environment variables
Diesel uses environment variables to connect to the database. To set the environment variables, create a file called `.env` in the root of the project based on the `default_env` file.

### Running the migrations
To run the migrations, run the following command:

```bash
diesel migration run
```

### Reverting the migrations
To revert the migrations, run the following command:

```bash
diesel migration revert --all
```

### Using the Diesel extended CLI
The Diesel extended CLI is a set of commands that can be used to print out the Rust struct associated with a table, saving a significant amount of time when writing the code. Note that such struct are not always correct and may need to be adjusted,
expecially when these structs include the use of less common types such as `Money` or `Interval`. To install the Diesel extended CLI, run the following command:

```bash
cargo install diesel_cli_ext
```

To use the Diesel extended CLI to generate the models, **AFTER HAVING BACKED UP THE PREVIOUS VERSION**, run the following command:

```bash
diesel_ext --model --add-table-name > src/models.rs
```

### Additional python utilities
In order to handle the many Diesel migrations that compose the application database, we have gradually created some Python utilities.

#### Increase directory counter
The increase directory counter script is a Python script that increases the counter of the migration directories. This is useful when you want to add a new migration directory to the project in the middle of the existing ones, and you are not keen on manually renaming 50+ directories. To use it, run the following command:

```bash
python migrations/increase_directory_counter.py $number_from_which_to_start
```

where `$number_from_which_to_start` is the number from which to start the counter. This will increase the counter of the migration directories starting from the number specified.

#### Inserting a new migration at a specific position
The insert migration script is a Python script that inserts a new migration at a specific position. This is useful when you want to add a new migration in the middle of the existing ones, and you are not keen on manually renaming 50+ directories. It will REVERT all of the migrations with counter greater or equal to than the one specified, but NOT reapply them. To use it, run the following command:

```bash
python migrations/insert_migration.py $number_from_which_to_start $migration_name
```

where `$number_from_which_to_start` is the number from which to start the counter. This will insert a new migration at the specified position.

## Starting cargo watch
Cargo watch is a tool that watches for changes in the project and automatically recompiles the project.

To install it, as per many other Rust tools, run the following command:

```bash
cargo install cargo-watch
```

To start cargo watch, run the following command:

```bash
cargo watch -q -c -w src/ -x run
```

### Common errors

#### Forgetting to start Postgres or Postgres Docker
If you forget to start Postgres or Postgres Docker, you will get the following error:

```bash
[2024-02-24T15:24:47Z ERROR r2d2] connection to server at "localhost" (::1), port 5432 failed: Connection refused
        Is the server running on that host and accepting TCP/IP connections?
    connection to server at "localhost" (127.0.0.1), port 5432 failed: Connection refused
        Is the server running on that host and accepting TCP/IP connections?
```

Start Postgres or Postgres Docker and the error will be resolved.

#### Unable to resolve GitHub
If you are unable to resolve GitHub, you will get the following error:

```bash
error: failed to query replaced source registry `crates-io`
```

It may mean you have some issues in the default DNS available to the Docker deamon.
In order to solve this issue, you can add the following lines to the `daemon.json` file:

```json
{ "dns" : [ "1.1.1.1" , "8.8.8.8" ] }
```

and restart the Docker deamon by running the following command (on Linux)

```bash
sudo systemctl restart docker
```