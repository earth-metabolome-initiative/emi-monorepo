#![doc = include_str!("../README.md")]

mod errors;
mod migration;
mod migration_directory;
mod migration_kind;

/// Prelude module with the most common types.
pub mod prelude {
    pub use crate::{
        errors::Error as MigrationError, migration::Migration,
        migration_directory::MigrationDirectory, migration_kind::MigrationKind,
    };
}
