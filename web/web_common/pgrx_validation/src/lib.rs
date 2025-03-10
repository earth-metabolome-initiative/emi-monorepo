#![doc = include_str!("../README.md")]

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

use pgrx_validation_derive::validation;

pub const EXTENSION_NAME: &str = "pgrx_validation";

#[validation]
/// Validates that the given value is not empty.
///
/// # Arguments
///
/// * `value` a string
///
/// # Errors
///
/// * `validation_errors::SingleFieldError::EmptyText` if the string is empty.
pub fn must_not_be_empty(value: &str) -> Result<(), validation_errors::SingleFieldError> {
    if value.is_empty() {
        Err(validation_errors::SingleFieldError::EmptyText(()))
    } else {
        Ok(())
    }
}

#[validation]
/// Validates that two provided strings are distinct.
///
/// # Arguments
///
/// * `left` a string
/// * `right` a string
///
/// # Errors
///
/// * `validation_errors::DoubleFieldError::NotDistinct((), ())` if the two
///   strings are equal.
pub fn must_be_distinct(
    left: &str,
    right: &str,
) -> Result<(), validation_errors::DoubleFieldError> {
    if left == right {
        Err(validation_errors::DoubleFieldError::NotDistinct((), ()))
    } else {
        Ok(())
    }
}

#[validation]
/// Validates that two provided integers are distinct.
///
/// # Arguments
///
/// * `left`: the left i32
/// * `right`: the right i32
///
/// # Errors
///
/// * `validation_errors::DoubleFieldError::NotDistinct((), ())` if the two
///   integers are equal.
pub fn must_be_distinct_i32(
    left: i32,
    right: i32,
) -> Result<(), validation_errors::DoubleFieldError> {
    if left == right {
        Err(validation_errors::DoubleFieldError::NotDistinct((), ()))
    } else {
        Ok(())
    }
}

#[validation]
/// Validates that two provided integers are distinct.
///
/// # Arguments
///
/// * `left`: the left uuid
/// * `right`: the right uuid
///
/// # Errors
///
/// * `validation_errors::DoubleFieldError::NotDistinct((), ())` if the two
///   integers are equal.
pub fn must_be_distinct_uuid(
    left: rosetta_uuid::Uuid,
    right: rosetta_uuid::Uuid,
) -> Result<(), validation_errors::DoubleFieldError> {
    if left == right {
        Err(validation_errors::DoubleFieldError::NotDistinct((), ()))
    } else {
        Ok(())
    }
}

#[validation]
/// Validates that the given value is a valid mail address.
///
/// # Arguments
///
/// * `value` a string
///
/// # Errors
///
/// * `validation_errors::SingleFieldError::InvalidMail` if the string is not a
///   valid mail address.
pub fn must_be_mail(value: &str) -> Result<(), validation_errors::SingleFieldError> {
    must_not_be_empty(value)?;
    if validator::validate_email(value) {
        Ok(())
    } else {
        Err(validation_errors::SingleFieldError::InvalidMail(()))
    }
}

#[validation]
/// Control that the f64 is strictly positive (0, ...].
///
/// # Arguments
///
/// * `value` a f64
///
/// # Errors
///
/// * `validation_errors::SingleFieldError::UnexpectedNegativeOrZeroValue(())`
///   if the value is negative or zero.
pub fn must_be_strictly_positive_f64(
    value: f64,
) -> Result<(), validation_errors::SingleFieldError> {
    if value > 0.0 {
        Ok(())
    } else {
        Err(validation_errors::SingleFieldError::UnexpectedNegativeOrZeroValue(()))
    }
}

#[validation]
/// Control that the float is strictly positive (0, ...].
pub fn must_be_strictly_positive_f32(
    value: f32,
) -> Result<(), validation_errors::SingleFieldError> {
    if value > 0.0 {
        Ok(())
    } else {
        Err(validation_errors::SingleFieldError::UnexpectedNegativeOrZeroValue(()))
    }
}

#[validation]
/// Control that the f32 is strictly greater than the provided value.
///
/// # Arguments
///
/// * `value` a f32
/// * `lower_bound` a f32
///
/// # Errors
///
/// * `validation_errors::SingleFieldError::TooLowValue(())` if the value is not
///   strictly greater than the lower bound.
pub fn must_be_strictly_greater_than_f32(
    value: f32,
    lower_bound: f32,
) -> Result<(), validation_errors::SingleFieldError> {
    if value > lower_bound {
        Ok(())
    } else {
        Err(validation_errors::SingleFieldError::TooLowValue((), lower_bound as f64))
    }
}

#[validation]
/// Control that the f32 is strictly smaller than the provided value.
///
/// # Arguments
///
/// * `value` a f32
/// * `lower_bound` a f32
///
/// # Errors
///
/// * `validation_errors::SingleFieldError::TooLowValue(())` if the value is not
///   strictly smaller than the lower bound.
pub fn must_be_strictly_smaller_than_f32(
    value: f32,
    lower_bound: f32,
) -> Result<(), validation_errors::SingleFieldError> {
    if value < lower_bound {
        Ok(())
    } else {
        Err(validation_errors::SingleFieldError::TooHighValue((), lower_bound as f64))
    }
}

#[validation]
/// Control that the f64 is strictly greater than the provided value.
///
/// # Arguments
///
/// * `value` a f64
/// * `lower_bound` a f64
///
/// # Errors
///
/// * `validation_errors::SingleFieldError::TooLowValue(())` if the value is not
///   strictly greater than the lower bound.
pub fn must_be_strictly_greater_than_f64(
    value: f64,
    lower_bound: f64,
) -> Result<(), validation_errors::SingleFieldError> {
    if value > lower_bound {
        Ok(())
    } else {
        Err(validation_errors::SingleFieldError::TooLowValue((), lower_bound))
    }
}

#[validation]
/// Control that the f64 is strictly smaller than the provided value.
///
/// # Arguments
///
/// * `value` a f64
/// * `lower_bound` a f64
///
/// # Errors
///
/// * `validation_errors::SingleFieldError::TooLowValue(())` if the value is not
///   strictly smaller than the lower bound.
pub fn must_be_strictly_smaller_than_f64(
    value: f64,
    lower_bound: f64,
) -> Result<(), validation_errors::SingleFieldError> {
    if value < lower_bound {
        Ok(())
    } else {
        Err(validation_errors::SingleFieldError::TooHighValue((), lower_bound))
    }
}

#[cfg(feature = "pgrx")]
#[cfg(any(test, feature = "pg_test"))]
#[cfg_attr(feature = "pgrx", pgrx::pg_schema)]
mod tests {
    use pgrx::prelude::*;

    use super::*;

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
        assert_eq!(
            must_not_be_empty(x).unwrap_err(),
            validation_errors::SingleFieldError::EmptyText(())
        );
        assert!(must_not_be_empty("marco").is_ok())
    }

    #[test]
    fn test_must_be_mail() {
        assert!(must_be_mail("marco.visani@unifr.ch").is_ok());
        assert_eq!(
            must_be_mail("marco").unwrap_err(),
            validation_errors::SingleFieldError::InvalidMail(())
        );
        assert_eq!(
            must_be_mail("").unwrap_err(),
            validation_errors::SingleFieldError::EmptyText(())
        );
    }

    #[test]
    fn test_must_be_strictly_positive_f64() {
        assert!(must_be_strictly_positive_f64(3.0_f64).is_ok());
        assert_eq!(
            must_be_strictly_positive_f64(-1.6_f64).unwrap_err(),
            validation_errors::SingleFieldError::UnexpectedNegativeOrZeroValue(())
        );
        assert_eq!(
            must_be_strictly_positive_f64(0.0_f64).unwrap_err(),
            validation_errors::SingleFieldError::UnexpectedNegativeOrZeroValue(())
        );
    }

    #[test]
    fn test_must_be_strictly_positive_f32() {
        assert!(must_be_strictly_positive_f32(3.0 as f32).is_ok());
        assert_eq!(
            must_be_strictly_positive_f32(-3.0 as f32).unwrap_err(),
            validation_errors::SingleFieldError::UnexpectedNegativeOrZeroValue(())
        );
        assert_eq!(
            must_be_strictly_positive_f32(0.0 as f32).unwrap_err(),
            validation_errors::SingleFieldError::UnexpectedNegativeOrZeroValue(())
        );
        assert_eq!(
            must_be_strictly_positive_f32(-0.0 as f32).unwrap_err(),
            validation_errors::SingleFieldError::UnexpectedNegativeOrZeroValue(())
        )
    }

    #[test]
    fn test_must_be_distinct() {
        assert!(must_be_distinct("marco", "visani").is_ok());
        assert_eq!(
            must_be_distinct("marco", "marco").unwrap_err(),
            validation_errors::DoubleFieldError::NotDistinct((), ())
        );
    }

    #[test]
    fn test_must_be_distinct_i32() {
        assert!(must_be_distinct_i32(3, 4).is_ok());
        assert_eq!(
            must_be_distinct_i32(3, 3).unwrap_err(),
            validation_errors::DoubleFieldError::NotDistinct((), ())
        );
    }

    #[test]
    fn test_must_be_distinct_uuid() {
        let uuid1 = uuid::Uuid::new_v4();
        let uuid2 = uuid::Uuid::new_v4();
        assert!(must_be_distinct_uuid(uuid1, uuid2).is_ok());
        assert_eq!(
            must_be_distinct_uuid(uuid1, uuid1).unwrap_err(),
            validation_errors::DoubleFieldError::NotDistinct((), ())
        );
    }

    #[test]
    fn test_must_be_strictly_greater_than_f64() {
        assert!(must_be_strictly_greater_than_f64(3.0_f64, 2.0_f64).is_ok());
        assert_eq!(
            must_be_strictly_greater_than_f64(3.0_f64, 3.0_f64).unwrap_err(),
            validation_errors::SingleFieldError::TooLowValue((), 3.0)
        );
        assert_eq!(
            must_be_strictly_greater_than_f64(3.0_f64, 4.0_f64).unwrap_err(),
            validation_errors::SingleFieldError::TooLowValue((), 4.0)
        );
    }

    #[test]
    fn test_must_be_strictly_smaller_than_f64() {
        assert!(must_be_strictly_smaller_than_f64(3.0_f64, 4.0_f64).is_ok());
        assert_eq!(
            must_be_strictly_smaller_than_f64(3.0_f64, 3.0_f64).unwrap_err(),
            validation_errors::SingleFieldError::TooHighValue((), 3.0)
        );
        assert_eq!(
            must_be_strictly_smaller_than_f64(3.0_f64, 2.0_f64).unwrap_err(),
            validation_errors::SingleFieldError::TooHighValue((), 2.0)
        );
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
