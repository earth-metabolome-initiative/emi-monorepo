//! Submodule providing cached methods for the table properties.

use cached::{UnboundCache, proc_macro::cached};
use inflector::Inflector;
use snake_case_sanitizer::Sanitizer as SnakeCaseSanizer;

use crate::errors::WebCodeGenError;

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
    Ok(sanitizer.to_camel_case(singular_name(name)?)?)
}

#[cached(
    result = true,
    ty = "UnboundCache<String, String>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ name.to_owned() }"#
)]
fn singular_name(name: &str) -> Result<String, WebCodeGenError> {
    // We split the table name by underscores and remove the last element.
    let mut parts = name.split('_').map(std::string::ToString::to_string).collect::<Vec<String>>();
    let last_element = parts.pop().unwrap();
    // We convert to singular form the last element and join the parts back
    // together.
    parts.push(Inflector::default().singularize(&last_element));
    Ok(parts.join("_"))
}

#[cached(
    result = true,
    ty = "UnboundCache<String, String>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ name.to_owned() }"#
)]
fn plural_name(name: &str) -> Result<String, WebCodeGenError> {
    // We split the table name by underscores and remove the last element.
    let mut parts = name.split('_').map(std::string::ToString::to_string).collect::<Vec<String>>();
    let last_element = parts.pop().unwrap();
    // We convert to singular form the last element and join the parts back
    // together.
    parts.push(Inflector::default().pluralize(&last_element));
    Ok(parts.join("_"))
}
