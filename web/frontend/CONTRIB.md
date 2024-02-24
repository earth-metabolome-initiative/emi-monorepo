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

#### Starting Trunk
To start Trunk, run the following command:

```bash
trunk serve --port 3000 --proxy-backend http://localhost:8080/api/
```

Where `--port` is the port on which to serve the application and `--proxy-backend` is the backend server to proxy requests to. The backend server is the server that serves the REST API.