# Use a lightweight image as a base
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app/backend

# Expose the port that the Actix server will run on
EXPOSE $ACTIX_PORT

# We need to install the `watch` command to run the Actix server
RUN cargo install cargo-watch

# We need to install postgresql-dev to build the diesel-cli
RUN apt-get update && apt-get install -y libpq-dev

# We need to compile diesel-cli to run the migrations
RUN cargo install diesel_cli --no-default-features --features postgres

# Command to run the Actix server application
CMD rm -rf /app/backend/backend.ready || true && touch /app/backend/backend.building && diesel migration run && cargo watch -q -c -w src/ -x run