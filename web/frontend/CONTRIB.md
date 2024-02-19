# How to contribute

## Setup

### Trunk (WebAssembly)
For the WebAssembly part of the project, we use Trunk to build and run the project. To get started with Trunk, as first thing we need to add the `wasm32-unknown-unknown` target to our Rust installation. A compiler target is a specific platform that Rust can compile to.

To do this, run the following command:

```bash
rustup target add wasm32-unknown-unknown
```

After that, we can install Trunk by running the following command. You may want to look up the latest version of Trunk on the [Yew website](https://yew.rs/docs/getting-started/introduction), so to check that the command below is up to date.

```bash
# note that this might take a while to install because it compiles everything from scratch
# Trunk also provides prebuilt binaries for a number of major package managers
# See https://trunkrs.dev/#install for further details
cargo install --locked trunk
```

### Installing cargo watch
What is cargo watch? It's a tool that watches for changes in your Rust project and automatically recompiles it. It's a great tool to have when you're working on a project and want to see the changes you've made without having to manually recompile it every time.

To install cargo watch, run the following command:

```bash
cargo install cargo-watch
```

You will be able to run cargo watch with the following command:

```bash
cargo watch -q -c -w src/ -x run
```

Note that this command should be made part of the script that we use to run the project in debug mode.
