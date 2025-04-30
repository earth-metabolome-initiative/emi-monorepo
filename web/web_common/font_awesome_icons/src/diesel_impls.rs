#![cfg(feature = "diesel")]
//! Submodule providing the diesel trait implementations for
//! [`Icon`](crate::Icon).

mod postgres;
mod sqlite;

/// The [`FAIcon`] SQL type.
///
/// # Implementation details
///
/// The `FAIcon` type is used to represent the a Font Awesome icon class
/// in PostgreSQL and SQLite databases.
#[derive(
    Debug, Clone, Copy, Default, diesel::query_builder::QueryId, diesel::sql_types::SqlType,
)]
#[cfg_attr(
    all(feature = "postgres", not(feature = "diesel-pgrx")),
    diesel(postgres_type(oid = 25, array_oid = 1009))
)]
#[cfg_attr(
    all(feature = "postgres", feature = "diesel-pgrx"),
    diesel(postgres_type(name = "FAIcon"))
)]
#[cfg_attr(feature = "sqlite", diesel(sqlite_type(name = "Text")))]
pub struct FAIcon;
