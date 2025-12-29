//! Returns a reference to the value at the last value in the provided
//! `ObjectName`.

use sqlparser::ast::{ObjectName, ObjectNamePart, ObjectNamePartFunction};

/// Returns a reference to the value at the last value in the provided
/// `ObjectName`.
///
/// # Arguments
///
/// * `object_name` - The `ObjectName` to extract the last part from.
///
/// # Panics
///
/// * Panics if the `ObjectName` has no parts.
pub fn last_str(object_name: &ObjectName) -> &str {
    match &object_name.0.last().expect("ObjectName has no parts") {
        ObjectNamePart::Identifier(ident) => ident.value.as_str(),
        ObjectNamePart::Function(ObjectNamePartFunction { name, .. }) => name.value.as_str(),
    }
}
