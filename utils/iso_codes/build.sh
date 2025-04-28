#!/bin/bash
set -e

PG_CONFIG_PATH=$(which pg_config)  # Auto-detect pg_config inside the container
OUTPUT_DIR="$HOME/extension"

# Ensure output directory exists
mkdir -p "$OUTPUT_DIR"

# We make sure that the following lines are present in the `Cargo.toml` file and have
# not been tampered with for any reason. If any of these lines are not present, we
# need to raise an appropriate error messages.

lines_to_check=(
    '# \[lib\]'
    '# crate-type = \[\"cdylib\", \"lib\"\]'
)

# Check if the lines are present in the Cargo.toml file
for line in "${lines_to_check[@]}"; do
	if ! grep -q "$line" Cargo.toml && ! grep -q "${line:2}" Cargo.toml; then
		echo "Error: Missing line in Cargo.toml: \`$line\`"
		exit 1
	fi
done

# We proceed to comment all of the non-commented lines in the `Cargo.toml` file
# listed above, and to uncomment the commented lines in the `Cargo.toml` file.

for line in "${lines_to_check[@]}"; do
	# Uncomment the line
	sed -i "s|$line|${line:2}|" Cargo.toml
done

# Initialize PGRX for PG 17
cargo pgrx init --pg17 "$PG_CONFIG_PATH"

# Build and package the extension
cargo pgrx package --pg-config "$PG_CONFIG_PATH" --out-dir "$OUTPUT_DIR"

# We restore the `Cargo.toml` file to its original state

for line in "${lines_to_check[@]}"; do
	# Comment the line
	sed -i "s|${line:2}|$line|" Cargo.toml
done

echo "Build complete. Shared object (.so) available in $OUTPUT_DIR"
