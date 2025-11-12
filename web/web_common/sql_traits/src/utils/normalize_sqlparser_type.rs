//! Submodule providing a function for normalizing `SQLParser` data types.

use sqlparser::ast::{DataType, ObjectName, ObjectNamePart, TimezoneInfo};

/// Normalizes `SQLParser` data types to a standard representation.
#[must_use]
#[inline]
pub fn normalize_sqlparser_type(sqlparser_type: &DataType) -> &str {
    match sqlparser_type {
        DataType::Uuid => "UUID",
        DataType::Text => "TEXT",
        DataType::Varchar(_) => "VARCHAR",
        DataType::Int(None) | DataType::Integer(None) => "INT",
        DataType::Real => "REAL",
        DataType::SmallInt(None) => "SMALLINT",
        DataType::Bool | DataType::Boolean => "BOOLEAN",
        DataType::Timestamp(None, TimezoneInfo::None) => "TIMESTAMP",
        DataType::Timestamp(None, TimezoneInfo::WithTimeZone) => "TIMESTAMPTZ",
        DataType::Custom(ObjectName(object_names), segments) => {
            if let [ObjectNamePart::Identifier(ident)] = object_names.as_slice() {
                if ident.value.as_str() == "GEOGRAPHY" && segments == &["Point", "4326"] {
                    return "GEOGRAPHY(Point, 4326)";
                }
                if ident.value.as_str() == "GEOMETRY" && segments == &["Point", "4326"] {
                    return "GEOMETRY(Point, 4326)";
                }
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
