# Use a lightweight image as a base
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app/frontend

RUN apt-get update && apt-get install -y tmux

# Expose the port that the Yew frontend will run on
EXPOSE $TRUNK_PORT

# We need to add the wasm compilation target to the image
RUN rustup target add wasm32-unknown-unknown

# We install the wasm-bindgen-cli tool to generate the JavaScript bindings for the WebAssembly code
# While this is not strictly necessary on most systems, it becomes mandatory when using Docker on
# an M1 Mac.
RUN cargo install --locked wasm-bindgen-cli

# Install the trunk package manager
RUN cargo install --locked trunk

# Command to run the Yew frontend application
CMD tmux new-session -s yew "trunk serve --port $TRUNK_PORT --address 0.0.0.0 --proxy-backend https://actix-server:${ACTIX_PORT}/api"
