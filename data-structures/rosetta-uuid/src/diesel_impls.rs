//! Implementations of the [`ToSql`](https://docs.rs/diesel/latest/diesel/serialize/trait.ToSql.html) and [`FromSql`](https://docs.rs/diesel/latest/diesel/deserialize/trait.FromSql.html) traits for [`Diesel`](https://docs.rs/diesel/latest/diesel/).

#[cfg(feature = "postgres")]
mod postgres;
#[cfg(feature = "sqlite")]
mod sqlite;
