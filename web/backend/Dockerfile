# Use a lightweight image as a base
FROM rust:latest

ARG ACTIX_PORT

# Expose the port that the Actix server will run on
EXPOSE $ACTIX_PORT

# We need to install postgresql-dev to build the diesel-cli
RUN apt-get update && apt-get install -y libpq-dev tmux

# We need to install the `watch` command to run the Actix server
RUN cargo install cargo-watch

# Set the working directory inside the container
WORKDIR /app/backend

# We run the Actix server by executing inside a TMUX session
# the bash script called `start.sh`
CMD tmux new-session -s actix-development 'bash /app/backend/start.sh'
