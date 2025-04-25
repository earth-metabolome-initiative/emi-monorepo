rustup update
cargo update
cd /home/appuser/app/web/frontend
trunk build --release
cd /home/appuser/app/web/backend
cargo run --release