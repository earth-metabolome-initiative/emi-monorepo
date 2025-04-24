# Use a lightweight image as a base
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app/frontend

RUN apt-get update && apt-get install -y tmux libnss3-tools build-essential procps curl file git

# Install brew for later installation of mkcert
RUN /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# We install mkcert to generate certificates for the Yew frontend
RUN brew install mkcert

# Expose the port that the Yew frontend will run on
EXPOSE $TRUNK_PORT

# We need to add the wasm compilation target to the image
RUN rustup target add wasm32-unknown-unknown

# Install the trunk package manager
RUN cargo install --locked trunk

# Install the certificates
RUN mkcert -install

# Command to run the Yew frontend application
CMD tmux new-session -s yew 'bash /app/frontend/start.sh'
