//! Provides a method returning the rust type which should be employed in the rust
//! struct to represent the type of a struct derived from or associated with a database table.

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
        "character varying" | "varchar" | "text" | "name" | "character" | "char"
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


/// Converts a `PostgreSQL` type to a `Diesel` type.
///
/// # Arguments
///
/// * `postgres_type` - A string slice that holds the `PostgreSQL` type name.
///
/// # Returns
///
/// A `Type` representing the corresponding Diesel type.
///
/// # Errors
///
/// * Returns an error if the type is not supported.
pub fn postgres_type_to_diesel_str(postgres_type: &str) -> Result<String, WebCodeGenError> {
    if let Some(postgres_type) = postgres_type.strip_suffix("[]") {
        return Ok(format!(
            "diesel::sql_types::Array<{}>",
            postgres_type_to_diesel_str(postgres_type)?
        ));
    }

    Ok(match postgres_type {
        // Numeric types
        "integer" | "i32" => "diesel::sql_types::Integer",
        "smallint" | "int2" => "diesel::sql_types::SmallInt",
        "bigint" | "int8" => "diesel::sql_types::BigInt",
        "real" | "float4" => "diesel::sql_types::Float",
        "double precision" | "float8" => "diesel::sql_types::Double",
        "money" => "diesel::pg::sql_types::Money",

        // Text types
        "text" | "character varying" | "name" | "cstring" => "diesel::sql_types::Text",
        "citext" => "diesel::sql_types::Citext",
        "char" | "character" => "diesel::sql_types::CChar",
        "bpchar" => "diesel::sql_types::Bpchar",

        // Boolean types
        "boolean" | "bool" => "diesel::sql_types::Bool",

        // Temporal types
        "timestamp without time zone" | "timestamp" => "diesel::sql_types::Timestamp",
        "timestamp with time zone" | "timestamptz" => {
            "rosetta_timestamp::diesel_impls::TimestampUTC"
        }
        "time" => "diesel::sql_types::Time",
        "date" => "diesel::sql_types::Date",
        "interval" => "diesel::sql_types::Interval",

        // Binary types
        "bytea" => "diesel::sql_types::Binary",

        // JSON types
        "json" => "diesel::sql_types::Json",
        "jsonb" => "diesel::sql_types::Jsonb",

        // Network address types
        "macaddr" => "diesel::sql_types::Macaddr",
        "inet" => "diesel::sql_types::Inet",

        // Object Identifier types
        "oid" => "diesel::sql_types::Oid",

        // Full-text search types
        "tsvector" => "diesel_full_text_search::TsVector",
        "tsquery" => "diesel_full_text_search::TsQuery",

        // GIS types
        "geometry" | "point" | "polygon" | "geometry(Point,4326)" | "line" => {
            "::postgis_diesel::sql_types::Geometry"
        }
        "geography" => "::postgis_diesel::sql_types::Geography",

        // Other
        "uuid" => "::rosetta_uuid::diesel_impls::Uuid",

        // ISO Codes
        "countrycode" | "CountryCode" => "::iso_codes::country_codes::diesel_impls::CountryCode",

        "mediatype" | "MediaType" => "::media_types::diesel_impls::MediaType",

        "cas" => "::cas_codes::diesel_impls::CAS",
        "molecularformula" => {
            "::molecular_formulas::molecular_formula::diesel_impls::MolecularFormula"
        }

        _ => {
            return Err(WebCodeGenError::UnknownDieselPostgresType(postgres_type.to_owned()));
        }
    }
    .to_owned())
}
