//! Crate to provide validations for:
//! 
//! * Postgres database, when compiling with `cargo pgrx package`
//! * Rust, when compiling with `cargo build`
//! 

#[cfg(feature = "pgrx")]
use pgrx::prelude::*;

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

#[cfg_attr(feature = "pgrx", pg_extern)]
fn hello_pgrx_validation() -> &'static str {
    "Hello, pgrx_validation"
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_pgrx_validation() {
        assert_eq!("Hello, pgrx_validation", crate::hello_pgrx_validation());
    }

}

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
