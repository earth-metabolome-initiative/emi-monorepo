//! Submodule providing general structs for representing database schemas.

pub mod generic_db;
pub use generic_db::{GenericDB, ParserDB};
pub mod metadata;

pub use metadata::{TableAttribute, TableMetadata};
