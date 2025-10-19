//! Utility functions and constants for code generation and string manipulation.

use cached::{UnboundCache, proc_macro::cached};
use inflector::Inflector;
use snake_case_sanitizer::Sanitizer;

/// Reserved Rust words that cannot be used as identifiers.
pub const RESERVED_RUST_WORDS: [&str; 49] = [
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "crate",
    "do", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in", "let",
    "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref", "return",
    "self", "static", "struct", "super", "trait", "true", "try", "type", "typeof", "unsafe",
    "unsized", "use", "virtual", "where", "while", "yield",
];

/// Returns whether the provided name is a reserved Rust word.
pub fn is_reserved_rust_word(name: &str) -> bool {
    debug_assert!(
        RESERVED_RUST_WORDS.windows(2).all(|w| w[0] < w[1]),
        "RESERVED_RUST_WORDS must be sorted"
    );
    RESERVED_RUST_WORDS.binary_search(&name).is_ok()
}

/// Diesel collisions that need to be handled.
pub const RESERVED_DIESEL_WORDS: [&str; 1] = ["columns"];

#[cached(
    ty = "UnboundCache<String, String>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ name.to_owned() }"#
)]
pub(crate) fn snake_case_name(name: &str) -> String {
    let sanitizer = Sanitizer::default()
        .include_defaults()
        .remove_leading_underscores()
        .remove_trailing_underscores();
    sanitizer.to_snake_case(name).unwrap_or_default()
}

#[cached(
    ty = "UnboundCache<String, String>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ name.to_owned() }"#
)]
pub(crate) fn singular_snake_name(name: &str) -> String {
    let sanitizer = Sanitizer::default()
        .include_defaults()
        .remove_leading_underscores()
        .remove_trailing_underscores();
    sanitizer.to_snake_case(&singular_name(name)).unwrap_or_default()
}

#[cached(
    ty = "UnboundCache<String, String>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ name.to_owned() }"#
)]
pub(crate) fn camel_case_name(name: &str) -> String {
    let sanitizer = Sanitizer::default()
        .include_defaults()
        .remove_leading_underscores()
        .remove_trailing_underscores();
    sanitizer.to_camel_case(name).unwrap_or_default()
}

#[cached(
    ty = "UnboundCache<String, String>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ name.to_owned() }"#
)]
pub(crate) fn struct_name(name: &str) -> String {
    let sanitizer = Sanitizer::default()
        .include_defaults()
        .remove_leading_underscores()
        .remove_trailing_underscores();
    sanitizer.to_camel_case(singular_name(name)).unwrap_or_default()
}

#[cached(
    ty = "UnboundCache<String, String>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ name.to_owned() }"#
)]
pub(crate) fn singular_name(name: &str) -> String {
    // We split the table name by underscores and remove the last element.
    let mut parts = name.split('_').map(std::string::ToString::to_string).collect::<Vec<String>>();
    let last = parts.pop().map(|last| Inflector::default().singularize(&last));
    parts.extend(last);
    parts.join("_")
}
