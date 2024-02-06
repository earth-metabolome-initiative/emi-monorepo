# How to fuzz
The fuzzer allows you to test the bindings with random inputs. It uses the `cargo-fuzz` crate to generate random inputs and test the bindings with them.

## Install cargo-fuzz
```bash
cargo install cargo-fuzz
```

## Run the fuzzer
```bash
cargo fuzz run random
```

You can stop the fuzzer at any time by pressing `Ctrl+C`. The fuzzer will print the inputs that caused the crash.

