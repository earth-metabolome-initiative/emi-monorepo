//! Submodule providing a function for normalizing `PostgreSQL` data types.

/// Normalizes `PostgreSQL` data types to a standard representation.
///
/// # Arguments
///
/// * `pg_type`: The `PostgreSQL` data type as a string slice.
///
/// # Returns
///
/// The normalized `PostgreSQL` data type as a string slice.
///
/// # Examples
///
/// ```rust
/// use sql_traits::utils::normalize_postgres_type;
///
/// let normalized = normalize_postgres_type("INT4");
/// assert_eq!(normalized, "INT");
/// ```
#[must_use]
#[inline]
pub fn normalize_postgres_type(pg_type: &str) -> &str {
    match pg_type.to_lowercase().trim_matches('\"') {
        "int2" | "smallint" | "smallserial" => "SMALLINT",
        "int4" | "integer" | "serial" => "INT",
        "int8" | "bigint" | "bigserial" => "BIGINT",
        "float4" | "real" => "real",
        "float8" | "double precision" => "double precision",
        "numeric" | "decimal" => "numeric",
        "bool" | "boolean" => "boolean",
        "varchar" | "character varying" => "VARCHAR",
        "char" | "character" => "CHAR",
        "text" => "TEXT",
        "date" => "date",
        "timestamp" | "timestamp without time zone" => "timestamp without time zone",
        "timestamptz" | "timestamp with time zone" => "timestamp with time zone",
        "time" | "time without time zone" => "time without time zone",
        "timetz" | "time with time zone" => "time with time zone",
        "bytea" => "bytea",
        _ => pg_type,
    }
}
