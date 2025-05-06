//! Submodule defining an enumeration of the possible default types for a
//! column.

use crate::errors::WebCodeGenError;

/// Enumeration of the possible default types for a column.
pub enum DefaultTypes {
    /// When the default value is the current timestamp.
    CurrentTimestamp,
    /// When the default value is a i16.
    I16(i16),
    /// When the default value is a i32.
    I32(i32),
    /// When the default value is a i64.
    I64(i64),
    /// When the default value is a boolean.
    Bool(bool),
    /// When the default value is a string.
    String(String),
}

impl core::fmt::Display for DefaultTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DefaultTypes::CurrentTimestamp => write!(f, "CURRENT_TIMESTAMP"),
            DefaultTypes::I16(value) => write!(f, "{value}_i16"),
            DefaultTypes::I32(value) => write!(f, "{value}_i32"),
            DefaultTypes::I64(value) => write!(f, "{value}_i64"),
            DefaultTypes::Bool(value) => write!(f, "{value}"),
            DefaultTypes::String(value) => write!(f, "{value}"),
        }
    }
}

impl DefaultTypes {
    pub fn new(column_rust_type: &str, default_value: &str) -> Result<Self, WebCodeGenError> {
        if default_value == "CURRENT_TIMESTAMP" {
            return Ok(DefaultTypes::CurrentTimestamp);
        }
        match column_rust_type {
            "i16" => {
                let value = default_value.parse::<i16>()?;
                Ok(DefaultTypes::I16(value))
            }
            "i32" => {
                let value = default_value.parse::<i32>()?;
                Ok(DefaultTypes::I32(value))
            }
            "i64" => {
                let value = default_value.parse::<i64>()?;
                Ok(DefaultTypes::I64(value))
            }
            "bool" => {
                let value = default_value.parse::<bool>()?;
                Ok(DefaultTypes::Bool(value))
            }
            "String" => {
                if let Some(value) =
                    default_value.strip_prefix('\'').and_then(|s| s.strip_suffix("\'::text"))
                {
                    Ok(DefaultTypes::String(value.to_string()))
                } else {
                    unimplemented!(
                        "Default type `{default_value}` for column type `{column_rust_type}` is not implemented",
                    );
                }
            }
            _ => {
                unimplemented!(
                    "Default type `{default_value}` for column type `{column_rust_type}` is not implemented",
                )
            }
        }
    }
}
