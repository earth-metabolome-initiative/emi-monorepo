
# from your workspace root:
cargo llvm-cov \
  --package molecular_formula \
  --tests \
  --html \
  --ignore-filename-regex '/\.cargo/registry/|/rustc/' \
  --open
