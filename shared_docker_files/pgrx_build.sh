#!/bin/bash
set -e

PG_CONFIG_PATH=$(which pg_config)  # Auto-detect pg_config inside the container
OUTPUT_DIR="$HOME/extension"

# Ensure output directory exists
mkdir -p "$OUTPUT_DIR"

# We make sure that the following lines are present in the `Cargo.toml` file and have
# not been tampered with for any reason. If any of these lines are not present, we
# need to raise an appropriate error messages.

lines_to_decomment=(
    '\[lib\]'
    'crate-type = \[\"cdylib\", \"lib\"\]'
)

lines_to_comment=(
	'\[lints\]'
	'workspace = true'
)

lines_to_check=("${lines_to_decomment[@]}" "${lines_to_comment[@]}")

# Check if the lines are present in the Cargo.toml file
for line in "${lines_to_check[@]}"; do
	if ! grep -q "^$line" Cargo.toml && ! grep -q "^# ${line}" Cargo.toml; then
		echo "Error: Missing line in Cargo.toml: \`$line\`"
		exit 1
	fi
done

# We proceed to comment all of the non-commented lines in the `Cargo.toml` file
# listed above, and to uncomment the commented lines in the `Cargo.toml` file.

for line in "${lines_to_decomment[@]}"; do
	# Uncomment the line if needed
	if grep -q "^# $line" Cargo.toml; then
		# Uncomment the line
		sed -i "s|^# $line|$line|" Cargo.toml
	fi
done

for line in "${lines_to_comment[@]}"; do
	# Comment the line if needed
	if grep -q "^$line" Cargo.toml; then
		# Comment the line
		sed -i "s|^$line|# $line|" Cargo.toml
	fi
done

# Initialize PGRX for PG 17
cargo pgrx init --pg17 "$PG_CONFIG_PATH"

# Build and package the extension
cargo pgrx package --pg-config "$PG_CONFIG_PATH" --out-dir "$OUTPUT_DIR"

# We restore the `Cargo.toml` file to its original state

for line in "${lines_to_decomment[@]}"; do
	# Comment the line
	sed -i "s|^${line:2}|$line|" Cargo.toml
done

for line in "${lines_to_comment[@]}"; do
	# Uncomment the line
	sed -i "s|^# $line|$line|" Cargo.toml
done