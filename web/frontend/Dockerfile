# Use a lightweight image as a base
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app/frontend

# Expose the port that the Yew frontend will run on
EXPOSE $TRUNK_PORT

# We need to add the wasm compilation target to the image
RUN rustup target add wasm32-unknown-unknown

# Install the trunk package manager
RUN cargo install --locked trunk

# Command to run the Yew frontend application
CMD trunk serve --port $TRUNK_PORT --proxy-backend http://localhost:$ACTIX_PORT/api
