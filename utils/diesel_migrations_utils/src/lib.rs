#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(unused_macro_rules)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![deny(unused_import_braces)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

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
