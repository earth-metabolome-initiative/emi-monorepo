//! Crate to provide validations for:
//!
//! * Postgres database, when compiling with `cargo pgrx package`
//! * Rust, when compiling with `cargo build`
//!

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

#[validation]
/// Validates that the given value is a valid mail address.
pub fn must_be_mail(value: &str) -> Result<(), validation_errors::Error> {
    must_not_be_empty(value)?;
    if validator::validate_email(value) {
        Ok(())
    } else {
        Err(validation_errors::Error::InvalidMail)
    }
}

#[validation]
/// Control that the double is strictly positive (0, ...].
///
/// # Arguments
///
/// * `value` a double
pub fn must_be_strictly_positive_double(value: f64) -> Result<(), validation_errors::Error> {
    if value > 0.0 {
        Ok(())
    } else {
        Err(validation_errors::Error::UnexpectedNegativeOrZeroValue)
    }
}

#[validation]
/// Control that the float is strictly positive (0, ...].
pub fn must_be_strictly_positive_float(value: f32) -> Result<(), validation_errors::Error> {
    if value > 0.0 {
        Ok(())
    } else {
        Err(validation_errors::Error::UnexpectedNegativeOrZeroValue)
    }
}

#[cfg(feature = "pgrx")]
#[cfg(any(test, feature = "pg_test"))]
#[cfg_attr(feature = "pgrx", pgrx::pg_schema)]
mod tests {
    use super::*;
    use pgrx::prelude::*;

    #[pg_test]
    #[should_panic]
    fn test_must_not_be_empty() {
        let x = "";
        must_not_be_empty(x);
    }
}

#[cfg(not(feature = "pgrx"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_must_not_be_empty() {
        let x = "";
        assert_eq!(must_not_be_empty(x).unwrap_err(), validation_errors::Error::EmptyText);
        assert!(must_not_be_empty("marco").is_ok())
    }

    #[test]
    fn test_must_be_mail() {
        assert!(must_be_mail("marco.visani@unifr.ch").is_ok());
        assert_eq!(must_be_mail("marco").unwrap_err(), validation_errors::Error::InvalidMail);
        assert_eq!(must_be_mail("").unwrap_err(), validation_errors::Error::EmptyText);
    }

    #[test]
    fn test_must_be_strictly_positive_double() {
        assert!(must_be_strictly_positive_double(3.0 as f64).is_ok());
        assert_eq!(
            must_be_strictly_positive_double(-1.6 as f64).unwrap_err(),
            validation_errors::Error::UnexpectedNegativeOrZeroValue
        );
        assert_eq!(
            must_be_strictly_positive_double(0.0 as f64).unwrap_err(),
            validation_errors::Error::UnexpectedNegativeOrZeroValue
        );
    }

    #[test]
    fn test_must_be_strictly_positive_float() {
        assert!(must_be_strictly_positive_float(3.0 as f32).is_ok());
        assert_eq!(
            must_be_strictly_positive_float(-3.0 as f32).unwrap_err(),
            validation_errors::Error::UnexpectedNegativeOrZeroValue
        );
        assert_eq!(
            must_be_strictly_positive_float(0.0 as f32).unwrap_err(),
            validation_errors::Error::UnexpectedNegativeOrZeroValue
        );
        assert_eq!(
            must_be_strictly_positive_float(-0.0 as f32).unwrap_err(),
            validation_errors::Error::UnexpectedNegativeOrZeroValue
        )
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
