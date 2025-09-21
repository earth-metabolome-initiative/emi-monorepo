//! Provides a method returning the rust type which should be employed in the rust
//! struct to represent the type of a struct derived from or associated with a database table.

/// Trait to be implemented by structs derived from database tables to indicate
/// which rust type should be used in the rust struct to represent their type.
pub trait RustType {
    /// The error type returned by the method.
    type Error;

    /// Returns the rust type as a syn::Type.
    fn rust_type(&self, conn: &mut diesel::PgConnection) -> Result<syn::Type, Self::Error>;
}

/// Returns the Syn rust type of the `PgType`.
///
/// # Arguments
///
/// * `optional` - A boolean indicating whether the type is optional.
/// * `conn` - The Postgres connection.
///
/// # Returns
///
/// A Result containing the Syn rust type of the `PgType`, or an error if
/// the type is not supported.
///
/// # Errors
///
/// * Returns an error if the provided database connection fails.
pub fn rust_type(&self, optional: bool, conn: &mut PgConnection) -> Result<Type, WebCodeGenError> {
    match rust_type_str(&self.typname, conn) {
        Ok(rust_type) => Ok(parse_str::<Type>(rust_type)?),
        Err(error) => {
            if self.is_composite() || self.is_enum() {
                let mut struct_name = format!(
                    "crate::{CODEGEN_DIRECTORY}::{CODEGEN_STRUCTS_MODULE}::{CODEGEN_TYPES_PATH}::{}::{}",
                    self.snake_case_name()?,
                    self.camelcased_name()
                );
                if optional {
                    struct_name = format!("Option<{struct_name}>");
                }

                Ok(parse_str::<Type>(&struct_name)?)
            } else if self.is_user_defined(conn)? {
                let Some(base_type) = self.base_type(conn)? else {
                    return Err(WebCodeGenError::MissingBaseType(Box::new(self.clone())));
                };
                base_type.rust_type(optional, conn)
            } else {
                Err(error)
            }
        }
    }
}

/// Returns the Syn rust type of the column.
///
/// # Arguments
///
/// * `type_name` - The name of the type.
///
/// # Returns
///
/// A string slice containing the rust type.
///
/// # Panics
///
/// Panics if the type is not supported.
pub fn rust_type_str<S: AsRef<str>>(
    type_name: S,
    conn: &mut PgConnection,
) -> Result<&'static str, WebCodeGenError> {
    Ok(match type_name.as_ref() {
        // Numeric types
        "integer" | "int4" => "i32",
        "smallint" | "int2" => "i16",
        "bigint" => "i64",
        "real" | "float4" => "f32",
        "double precision" | "float8" | "numeric" => "f64",
        "money" => "BigDecimal",
        "oid" => "u32",

        // Text types
        "character varying" | "varchar" | "text" | "name" | "xml" | "character" | "char"
        | "citext" => "String",

        // Boolean types
        "boolean" | "bool" => "bool",

        // Temporal types
        "timestamp without time zone" => "::chrono::NaiveDateTime",
        "timestamp with time zone" | "timestamptz" => {
            let time_zone = PgSetting::time_zone(conn)?;
            match time_zone.setting.as_str() {
                "UTC" => "::rosetta_timestamp::TimestampUTC",
                unknown_time_zone => {
                    unimplemented!("Time zone `{unknown_time_zone}` not supported")
                }
            }
        }
        "date" => "::chrono::NaiveDate",
        "interval" => "::chrono::Duration",

        // Binary types
        "bytea" | "bit" | "bit varying" => "Vec<u8>",

        // JSON types
        "json" | "jsonb" => "::serde_json::Value",

        // Network address types
        "inet" | "cidr" => "std::net::IpAddr",
        "macaddr" | "macaddr8" => "std::net::MacAddr",

        // GIS types
        "point" => "::postgis_diesel::types::Point",
        "geometry" => panic!("Geometry type not supported"),

        // UUID type
        "uuid" => "::rosetta_uuid::Uuid",

        // ISO Codes
        "countrycode" | "CountryCode" => "::iso_codes::CountryCode",

        "cas" => "::cas_codes::CAS",
        "molecularformula" => "::molecular_formulas::MolecularFormula",

        "mediatype" | "MediaType" => "::media_types::MediaType",

        other => return Err(WebCodeGenError::UnknownPostgresRustType(other.to_owned())),
    })
}
