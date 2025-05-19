#![cfg(feature = "diesel")]
//! Submodule providing the diesel implementations for country codes.

mod postgres;
mod sqlite;

/// The [`CountryCode`] SQL type.
///
/// # Implementation details
///
/// The `CountryCode` type is used to represent the `country_code` type in
/// `PostgreSQL` and `SQLite` databases. In `PostgreSQL`, it uses as backend type
/// the `VARCHAR` type with a length of 2. Unfortunately, we cannot enforce
/// the length `2` in the type definition, and as such we will have to check
/// for this in the conversion.
///
/// In `SQLite`, the type is defined as `Text`.
#[derive(
    Debug, Clone, Copy, Default, diesel::query_builder::QueryId, diesel::sql_types::SqlType,
)]
#[cfg_attr(
    all(feature = "postgres", not(feature = "diesel_pgrx")),
    diesel(postgres_type(oid = 1043, array_oid = 1015))
)]
#[cfg_attr(
    all(feature = "postgres", feature = "diesel_pgrx"),
    diesel(postgres_type(name = "CountryCode"))
)]
#[cfg_attr(feature = "sqlite", diesel(sqlite_type(name = "Text")))]
pub struct CountryCode;
