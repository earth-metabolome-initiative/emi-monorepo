mkdir -p $DOCUMENTS_DIRECTORY
cd /app/frontend
trunk build --release
cd /app/backend
cargo run --release