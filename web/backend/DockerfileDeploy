# Use a lightweight image as a base
FROM rust:latest

# Expose the port that the Actix server will run on
EXPOSE $ACTIX_PORT

# We need to install postgresql-dev to build the diesel-cli
RUN apt-get update && apt-get install -y libpq-dev

# We need to install the `watch` command to run the Actix server
RUN cargo install cargo-watch

# We need to add the wasm compilation target to the image
RUN rustup target add wasm32-unknown-unknown

# Install the trunk package manager
RUN cargo install --locked trunk

# We need to compile diesel-cli to run the migrations
RUN cargo install diesel_cli --no-default-features --features postgres

# Command to run the Actix server application
CMD rm -rf /app/backend/backend.ready || true && \
    rm -rf /app/backend/backend.building || true && \
    touch /app/backend/backend.building && \
    cd /app/frontend && \
    echo "Building frontend..." >> /app/backend/backend.building && \
    trunk build --release && \
    echo "Building frontend... done" >> /app/backend/backend.ready && \
    echo "Copying frontend to backend..." >> /app/backend/backend.building && \
    cd /app/backend && \
    echo "Running Diesel migrations..." >> /app/backend/backend.ready && \
    diesel migration run && \
    echo "Running Diesel migrations... done" >> /app/backend/backend.ready && \
    echo "Building and running Actix server..." >> /app/backend/backend.ready && \
    cargo run --release