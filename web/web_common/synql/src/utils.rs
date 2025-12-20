//! Utility functions for code generation and string manipulation.
mod is_reserved_rust_word;
use cached::{UnboundCache, proc_macro::cached};
pub use is_reserved_rust_word::*;
use snake_case_sanitizer::Sanitizer;
use inflector::Inflector;

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
    snake_case_name(&singular_name(name))
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
pub(crate) fn singular_camel_case_name(name: &str) -> String {
    camel_case_name(&singular_name(name))
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
