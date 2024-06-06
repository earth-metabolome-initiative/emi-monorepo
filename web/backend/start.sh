rm -rf /app/backend/backend.ready || true
touch /app/backend/backend.building
cargo clean
export RUST_LOG=debug
cargo watch -q -c -w src/ -x run