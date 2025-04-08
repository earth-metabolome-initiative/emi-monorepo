rustup update
cargo update
cd /app/frontend
trunk build --release
cd /app/backend
cargo run --release