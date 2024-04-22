# Use a lightweight image as a base
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app/frontend

RUN apt-get update && apt-get install -y tmux

# Expose the port that the Yew frontend will run on
EXPOSE $TRUNK_PORT

# We need to add the wasm compilation target to the image
RUN rustup target add wasm32-unknown-unknown

# Install the trunk package manager
RUN cargo install --locked trunk

# Command to run the Yew frontend application
CMD tmux new-session -s yew "trunk serve --port $TRUNK_PORT --tls-key-path ../nginx/${DOMAIN}-key.pem --tls-cert-path ../nginx/${DOMAIN}.pem --address 0.0.0.0 --proxy-backend https://actix-server:${ACTIX_PORT}/api"
