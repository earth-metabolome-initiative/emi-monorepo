# Contributing to WebCodeGen

This document describes how to contribute to the WebCodeGen project.

## Setup

### Install Rust

You can install Rust by running the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install PostgreSQL

Before installing Diesel, you need to install PostgreSQL. Note that without having installed PostgreSQL, Diesel will not compile and fail with a linking error:

```bash
linking with `cc` failed: exit code: 1
```

Depending on your OS, you can install it with the following commands:

#### MacOS

On MacOS, you can install PostgreSQL with Homebrew:

```bash
brew install postgresql
```

#### Ubuntu

On Ubuntu, you can install PostgreSQL with the following commands:

```bash
sudo apt-get update && sudo apt-get install -y libpq-dev
```

#### Arch Linux

On Arch Linux, you can install PostgreSQL with the following commands:

```bash
sudo pacman -S postgresql
```
