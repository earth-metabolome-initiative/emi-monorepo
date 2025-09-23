//! Submodule providing traits used throughout the `pg_diesel` crate.

pub mod oid;
pub use oid::HasOid;
pub mod postgres_type;
pub use postgres_type::PostgresType;
