//! Provides a method returning the diesel type which should be employed in the diesel
//! schema to represent the type of a struct derived from or associated with a database table.

/// Trait to be implemented by structs derived from database tables to indicate
/// which diesel type should be used in the diesel schema to represent their type.
pub trait DieselType {}

/// Returns the Syn postgres type of the `PgType`.
    ///
    /// # Arguments
    ///
    /// * `nullable` - A boolean indicating whether the type is nullable.
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing the Syn postgres type of the `PgType`, or an error
    /// if the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn diesel_type(
        &self,
        nullable: bool,
        conn: &mut PgConnection,
    ) -> Result<Type, WebCodeGenError> {
        let diesel_type = postgres_type_to_diesel(&self.typname, nullable);
        match diesel_type {
            Ok(diesel_type) => Ok(diesel_type),
            Err(error) => {
                if self.is_composite() || self.is_enum() {
                    let snake_case_name = self.snake_case_name()?;
                    let mut full_name = format!(
                        "crate::{CODEGEN_DIRECTORY}::{CODEGEN_DIESEL_MODULE}::{CODEGEN_TYPES_PATH}::{snake_case_name}::{}",
                        &self.pg_binding_name()
                    );
                    if nullable {
                        full_name = format!("diesel::sql_types::Nullable<{full_name}>");
                    }
                    Ok(parse_str::<Type>(&full_name)?)
                } else if self.is_user_defined(conn)? {
                    let base_type = self.base_type(conn)?;
                    if let Some(base_type) = base_type {
                        base_type.diesel_type(nullable, conn)
                    } else {
                        Err(WebCodeGenError::MissingBaseType(Box::new(self.clone())))
                    }
                } else {
                    Err(error)
                }
            }
        }
    }


/// Converts a `PostgreSQL` type to a `Diesel` type.
///
/// # Arguments
///
/// * `postgres_type` - A string slice that holds the `PostgreSQL` type name.
/// * `nullable` - A boolean indicating whether the type is nullable.
///
/// # Returns
///
/// A `Type` representing the corresponding Diesel type.
///
/// # Errors
///
/// * Returns an error if the type is not supported.
pub fn postgres_type_to_diesel(
    postgres_type: &str,
    nullable: bool,
) -> Result<Type, WebCodeGenError> {
    let mut rust_type_str = postgres_type_to_diesel_str(postgres_type)?;

    if nullable {
        rust_type_str = format!("diesel::sql_types::Nullable<{rust_type_str}>");
    }

    Ok(parse_str::<Type>(&rust_type_str)?)
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

