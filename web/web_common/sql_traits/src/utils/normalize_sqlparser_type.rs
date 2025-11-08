//! Submodule providing a function for normalizing SQLParser data types.

use sqlparser::ast::{DataType, ObjectName, ObjectNamePart};

/// Normalizes SQLParser data types to a standard representation.
pub fn normalize_sqlparser_type(sqlparser_type: &DataType) -> &str {
    match sqlparser_type {
        DataType::Uuid => "UUID",
        DataType::Text => "TEXT",
        DataType::Varchar(_) => "VARCHAR",
        DataType::Int(None) | DataType::Integer(None) => "INT",
        DataType::Real => "REAL",
        DataType::Custom(ObjectName(object_names), segments) => {
            if let [ObjectNamePart::Identifier(ident)] = object_names.as_slice() {
                ident.value.as_str()
            } else {
                unimplemented!(
                    "Normalization for custom SQLParser data type `{sqlparser_type:?}` is not yet implemented for object names `{object_names:?}` and segments `{segments:?}`"
                )
            }
        }
        _ => {
            unimplemented!(
                "Normalization for SQLParser data type `{sqlparser_type:?}` is not yet implemented `{}`",
                sqlparser_type.to_string()
            )
        }
    }
}
