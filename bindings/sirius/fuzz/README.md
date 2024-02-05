# How to fuzz
The fuzzer allows you to test the bindings with random inputs. It uses the `cargo-fuzz` crate to generate random inputs and test the bindings with them.

```bash
cargo fuzz run random
```

