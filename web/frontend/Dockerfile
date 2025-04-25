# Use a lightweight image as a base
FROM rust:latest

ARG HOST_UID
ARG HOST_GID

# Set the working directory inside the container

RUN apt-get update && apt-get install -y tmux

# Expose the port that the Yew frontend will run on
EXPOSE $TRUNK_PORT

# Create non-root user
RUN groupadd -g ${HOST_GID} appgroup && \
    useradd -m -u ${HOST_UID} -g ${HOST_GID} appuser

# Switch to non-root user
USER appuser
WORKDIR /home/appuser/app/web/frontend

# Add .cargo/bin to PATH
ENV PATH="/home/appuser/.cargo/bin:${PATH}"

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

# Command to run the Yew frontend application
CMD tmux new-session -s yew 'bash /home/appuser/app/web/frontend/start.sh'
