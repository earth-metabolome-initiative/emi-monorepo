FROM postgres:17

# Install dependencies for Rust and PGRX
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    clang \
    cmake \
    git \
    libssl-dev \
    pkg-config \
	libpq-dev \
	postgresql-server-dev-17 \
    && rm -rf /var/lib/apt/lists/*

# Get Rust;
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Install PGRX
RUN cargo install cargo-pgrx --locked
