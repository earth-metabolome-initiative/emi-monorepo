//! Utility functions and constants for code generation and string manipulation.

/// Reserved Rust words that cannot be used as identifiers.
pub const RESERVED_RUST_WORDS: [&str; 49] = [
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "crate",
    "do", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in", "let",
    "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref", "return",
    "self", "static", "struct", "super", "trait", "true", "try", "type", "typeof", "unsafe",
    "unsized", "use", "virtual", "where", "while", "yield",
];

/// Returns whether the provided name is a reserved Rust word.
#[inline]
#[must_use]
pub fn is_reserved_rust_word(name: &str) -> bool {
    debug_assert!(
        RESERVED_RUST_WORDS.windows(2).all(|w| w[0] < w[1]),
        "RESERVED_RUST_WORDS must be sorted"
    );
    RESERVED_RUST_WORDS.binary_search(&name).is_ok()
}
