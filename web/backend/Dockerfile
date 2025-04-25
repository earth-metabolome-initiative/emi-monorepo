# Use a lightweight image as a base
FROM ubuntu:22.04

ARG ACTIX_PORT
ARG HOST_UID
ARG HOST_GID

# Expose the port that the Actix server will run on
EXPOSE $ACTIX_PORT

# We need to install postgresql-dev to build the diesel-cli
RUN apt update && apt install -y libpq-dev tmux libnss3-tools build-essential procps curl file git wget mkcert pkg-config

# Create non-root user
RUN groupadd -g ${HOST_GID} appgroup && \
    useradd -m -u ${HOST_UID} -g ${HOST_GID} appuser

# Switch to non-root user
USER appuser
# Set the working directory inside the container
WORKDIR /home/appuser/app/web/backend

# Add Go to PATH
ENV PATH="/usr/local/go/bin:${PATH}"

# Get Rust;
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add .cargo/bin to PATH
ENV PATH="/home/appuser/.cargo/bin:${PATH}"

RUN cargo install cargo-watch

# We run the Actix server by executing inside a TMUX session
# the bash script called `start.sh`
CMD tmux new-session -s actix-development 'bash /home/appuser/app/web/backend/start.sh'
