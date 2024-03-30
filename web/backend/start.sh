mkdir -p $DOCUMENTS_DIRECTORY
rm -rf /app/backend/backend.ready || true
touch /app/backend/backend.building
cargo clean
diesel migration run
export RUST_LOG=debug
cargo watch -q -c -w src/ -x run