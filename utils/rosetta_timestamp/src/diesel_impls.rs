#![cfg(feature = "diesel")]
//! Submodule proving the implementations of the diesel-related traits.

mod postgres;
mod sqlite;

/// The [`TimestampUTC`] SQL type.
///
/// ### [`ToSql`] impls
///
/// - [`chrono::DateTime<chrono::Utc>`][chrono::DateTime]
///
/// ### [`FromSql`] impls
///
/// - [`chrono::DateTime<chrono::Utc>`][chrono::DateTime]
///
/// [`ToSql`]: crate::serialize::ToSql
/// [`FromSql`]: crate::deserialize::FromSql
#[derive(
    Debug, Clone, Copy, Default, diesel::query_builder::QueryId, diesel::sql_types::SqlType,
)]
#[cfg_attr(feature = "postgres", diesel(postgres_type(oid = 1184, array_oid = 1185)))]
#[cfg_attr(feature = "sqlite", diesel(sqlite_type(name = "Text")))]
pub struct TimestampUTC;
