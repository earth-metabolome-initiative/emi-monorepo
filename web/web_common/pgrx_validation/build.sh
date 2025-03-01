#!/bin/bash
set -e

PG_CONFIG_PATH=$(which pg_config)  # Auto-detect pg_config inside the container
OUTPUT_DIR="/workspace/extension"

# Ensure output directory exists
mkdir -p "$OUTPUT_DIR"

# Initialize PGRX for PG 17
cargo pgrx init --pg17 "$PG_CONFIG_PATH"

# Build and package the extension
cargo pgrx package --pg-config "$PG_CONFIG_PATH" --out-dir "$OUTPUT_DIR"

echo "Build complete. Shared object (.so) available in $OUTPUT_DIR"
