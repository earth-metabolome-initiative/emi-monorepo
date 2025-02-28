# How to get started

How to get started working on the [`pgrx`-based extensions](https://github.com/pgcentralfoundation/pgrx).

## Prerequisites

The installation procedure, [other than the Rust toolchain](https://www.rust-lang.org/tools/install), requires also a number of additional requirements, which include the [`readline`](https://tiswww.cwru.edu/php/chet/readline/rltop.html) library, the [`bison`](https://www.gnu.org/software/bison/) parser generator, and the [`flex`](https://github.com/westes/flex) lexical analyzer.

On Ubuntu, this can be done with:

```bash
sudo apt update
sudo apt -y install libreadline-dev bison flex
```

Since we will be using `Docker` for testing, it is also necessary to have it installed. Please install it by following [the current version of the short tutorial made available here](https://github.com/LucaCappelletti94/linux-setup/blob/main/DOCKER.md).

## Installation

First, install the `cargo-pgrx` tool:

```bash
cargo install --locked cargo-pgrx
```

Next, once per installation, it is necessary to initialize it by running the init command:

```bash
cargo pgrx init
```

which will identify the existing installations of PostgreSQL on your system. It may take a
while to complete, as it will compile PGRX against each of the identified installations.

## Getting started with extensions

To then create a new extension, in a context where you DO NOT already have a crate, you can run:

```bash
cargo pgrx new my_extension
```

## Building the extension

To build the extension, you can run:

```bash
cargo pgrx package
```

This will by default as output in the `./target/[debug|release]/extname-pgXX/` directory.

To build the extension in an arbitrary directory:

```bash
cargo pgrx package --out-dir my_own_extension
```

## Testing the extension

After having built the extension in a directory such as `my_own_extension` as described above, you can run the test suite which will create a temporary docker container with PostgreSQL by using [`testcontainers`](https://testcontainers.com/). It will then copy the `lib` and `extension` directories in the container by using `docker cp` and run the test migration using `cargo test`:

```bash
cargo test --test test_migration
```
