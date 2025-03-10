//! Implementations of the [`ToSql`](https://docs.rs/diesel/latest/diesel/serialize/trait.ToSql.html) and [`FromSql`](https://docs.rs/diesel/latest/diesel/deserialize/trait.FromSql.html) traits for [`Diesel`](https://docs.rs/diesel/latest/diesel/).

#[cfg(feature = "postgres")]
mod postgres;
#[cfg(feature = "sqlite")]
mod sqlite;

/// The [`UUID`] SQL type.
///
/// ### [`ToSql`] impls
///
/// - [`uuid::Uuid`][Uuid]
///
/// ### [`FromSql`] impls
///
/// - [`uuid::Uuid`][Uuid]
///
/// [`ToSql`]: crate::serialize::ToSql
/// [`FromSql`]: crate::deserialize::FromSql
/// [Uuid]: https://docs.rs/uuid/*/uuid/struct.Uuid.html
/// [`UUID`]: https://www.postgresql.org/docs/current/datatype-uuid.html
#[derive(
    Debug, Clone, Copy, Default, diesel::query_builder::QueryId, diesel::sql_types::SqlType,
)]
#[cfg_attr(feature = "postgres", diesel(postgres_type(oid = 2950, array_oid = 2951)))]
#[cfg_attr(feature = "sqlite", diesel(sqlite_type(name = "Binary")))]
pub struct Uuid;
