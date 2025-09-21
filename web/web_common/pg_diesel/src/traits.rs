//! Submodule providing traits used throughout the `pg_diesel` crate.

pub mod pg_model;
pub use pg_model::PgModel;
pub mod oid;
pub use oid::HasOid;