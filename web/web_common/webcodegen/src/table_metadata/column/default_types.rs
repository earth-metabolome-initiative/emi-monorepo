//! Submodule defining an enumeration of the possible default types for a
//! column.

use proc_macro2::TokenStream;

use crate::errors::WebCodeGenError;

#[derive(Debug, Clone)]
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
    /// When the default value is a UUID.
    Uuid(TokenStream),
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
            "rosetta_uuid::Uuid" => {
                match default_value {
                    "gen_random_uuid()" => {
                        Ok(DefaultTypes::Uuid(quote::quote! {rosetta_uuid::Uuid::new_v4()}))
                    }
                    _ => {
                        unimplemented!(
                            "Default type `{default_value}` for column type `{column_rust_type}` is not implemented",
                        )
                    }
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
