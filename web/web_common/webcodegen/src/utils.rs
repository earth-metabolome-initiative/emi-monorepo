//! Submodule providing cached methods for the table properties.

use cached::{UnboundCache, proc_macro::cached};
use inflector::Inflector;
use snake_case_sanitizer::Sanitizer as SnakeCaseSanizer;

use crate::errors::WebCodeGenError;

/// Reserved Rust words that cannot be used as identifiers.
pub const RESERVED_RUST_WORDS: [&str; 49] = [
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", // Future reserved keywords
    "abstract", "async", "await", "become", "box", "do", "final", "macro", "override", "priv",
    "try", "typeof", "unsized", "virtual", "yield",
];

/// Diesel collisions that need to be handled.
pub const RESERVED_DIESEL_WORDS: [&str; 1] = ["columns"];

#[cached(
    result = true,
    ty = "UnboundCache<String, String>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ name.to_owned() }"#
)]
pub(crate) fn snake_case_name(name: &str) -> Result<String, WebCodeGenError> {
    let sanitizer = SnakeCaseSanizer::default()
        .include_defaults()
        .remove_leading_underscores()
        .remove_trailing_underscores();
    Ok(sanitizer.to_snake_case(name)?)
}

#[cached(
    result = true,
    ty = "UnboundCache<String, String>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ name.to_owned() }"#
)]
pub(crate) fn camel_case_name(name: &str) -> Result<String, WebCodeGenError> {
    let sanitizer = SnakeCaseSanizer::default()
        .include_defaults()
        .remove_leading_underscores()
        .remove_trailing_underscores();
    Ok(sanitizer.to_camel_case(name)?)
}

#[cached(
    result = true,
    ty = "UnboundCache<String, String>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ name.to_owned() }"#
)]
pub(crate) fn struct_name(name: &str) -> Result<String, WebCodeGenError> {
    let sanitizer = SnakeCaseSanizer::default()
        .include_defaults()
        .remove_leading_underscores()
        .remove_trailing_underscores();
    Ok(sanitizer.to_camel_case(singular_name(name))?)
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
