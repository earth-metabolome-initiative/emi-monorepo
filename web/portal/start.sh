# Script to start the portal frontend.
trunk serve --port 3000 > /dev/null 2>&1 &
cargo watch -q -c -w src/ -x run > /dev/null 2>&1 &
echo $! > /tmp/portal.pid
