//! Crate to provide validations for:
//! 
//! * Postgres database, when compiling with `cargo pgrx package`
//! * Rust, when compiling with `cargo build`
//! 

#[cfg(feature = "pgrx")]
use pgrx::prelude::*;

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

use pgrx_validation_derive::validation;

pub const EXTENSION_NAME: &str = "pgrx_validation";

#[validation]
/// Validates that the given value is not empty.
pub fn must_not_be_empty(value: &str) -> Result<(), validation_errors::Error> {
    if value.is_empty() {
        Err(validation_errors::Error::EmptyText)
    } else {
        Ok(())
    }
}

// #[cfg(any(test, feature = "pg_test"))]
// #[pg_schema]
// mod tests {
//     use pgrx::prelude::*;

//     #[pg_test]
//     fn test_hello_pgrx_validation() {
//         assert_eq!("Hello, pgrx_validation", crate::hello_pgrx_validation());
//     }

// }

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
