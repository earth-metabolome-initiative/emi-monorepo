# Fuzzing harnesses for molecular formulas

This crate provides harnesses to run fuzzing on the `molecular_formulas` crate.

## Setup

Install the dependency for [honggfuzz](https://docs.rs/honggfuzz/latest/honggfuzz/) for a linux system:

```bash
sudo apt install build-essential binutils-dev libunwind-dev
```

Next, install [honggfuzz](https://docs.rs/honggfuzz/latest/honggfuzz/) itself:

```bash
cargo install honggfuzz
```

## Available harnesses

### `FromStr`

The `FromStr` trait implementation is one of the most commonly used method to construct a molecular formula. As such, it is worth fuzzing extensively.

```bash
cargo hfuzz run from_str
```

and to run the crash cases:

```bash
cargo hfuzz run-debug from_str hfuzz_workspace/*/*.fuzz
```
