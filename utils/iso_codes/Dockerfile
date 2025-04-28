FROM postgres:17

ARG HOST_UID
ARG HOST_GID

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

# Create non-root user
RUN groupadd -g ${HOST_GID} appgroup && \
    useradd -m -u ${HOST_UID} -g ${HOST_GID} appuser

# Switch to non-root user
USER appuser
WORKDIR /home/appuser

# Add .cargo/bin to PATH
ENV PATH="/home/appuser/.cargo/bin:${PATH}"

# Get Rust;
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Install PGRX
RUN cargo install cargo-pgrx --locked
