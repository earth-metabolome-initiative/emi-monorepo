//! Submodule for float validation

use pgrx_validation_derive::validation;

#[validation]
/// Control that the float is strictly positive (0, ...].
///
/// # Arguments
///
/// * `value` a f32
///
/// # Errors
///
/// * `validation_errors::SingleFieldError::UnexpectedNegativeOrZeroValue(())`
///   if the value is negative or zero.
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
/// Control that the f32 is greater than the provided value.
///
/// # Arguments
///
/// * `value` a f32
/// * `lower_bound` a f32
///
/// # Errors
///
/// * `validation_errors::SingleFieldError::MustBeGreaterThan(())` if the value
///   is not greater than the lower bound.
pub fn must_be_greater_than_f32(
    value: f32,
    lower_bound: f32,
) -> Result<(), validation_errors::SingleFieldError> {
    if value >= lower_bound {
        Ok(())
    } else {
        Err(validation_errors::SingleFieldError::MustBeGreaterThan((), f64::from(lower_bound)))
    }
}

#[validation]
/// Control that the f32 is smaller than the provided value.
///
/// # Arguments
///
/// * `value` a f32
/// * `lower_bound` a f32
///
/// # Errors
///
/// * `validation_errors::SingleFieldError::MustBeGreaterThan(())` if the value
///   is not smaller than the lower bound.
pub fn must_be_smaller_than_f32(
    value: f32,
    lower_bound: f32,
) -> Result<(), validation_errors::SingleFieldError> {
    if value <= lower_bound {
        Ok(())
    } else {
        Err(validation_errors::SingleFieldError::MustBeSmallerThan((), f64::from(lower_bound)))
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
/// * `validation_errors::SingleFieldError::MustBeGreaterThan(())` if the value
///   is not strictly greater than the lower bound.
pub fn must_be_strictly_greater_than_f32(
    value: f32,
    lower_bound: f32,
) -> Result<(), validation_errors::SingleFieldError> {
    if value > lower_bound {
        Ok(())
    } else {
        Err(validation_errors::SingleFieldError::MustBeGreaterThan((), f64::from(lower_bound)))
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
/// * `validation_errors::SingleFieldError::MustBeGreaterThan(())` if the value
///   is not strictly smaller than the lower bound.
pub fn must_be_strictly_smaller_than_f32(
    value: f32,
    lower_bound: f32,
) -> Result<(), validation_errors::SingleFieldError> {
    if value < lower_bound {
        Ok(())
    } else {
        Err(validation_errors::SingleFieldError::MustBeSmallerThan((), f64::from(lower_bound)))
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
/// * `validation_errors::SingleFieldError::MustBeGreaterThan(())` if the value
///   is not strictly greater than the lower bound.
pub fn must_be_strictly_greater_than_f64(
    value: f64,
    lower_bound: f64,
) -> Result<(), validation_errors::SingleFieldError> {
    if value > lower_bound {
        Ok(())
    } else {
        Err(validation_errors::SingleFieldError::MustBeGreaterThan((), lower_bound))
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
/// * `validation_errors::SingleFieldError::MustBeGreaterThan(())` if the value
///   is not strictly smaller than the lower bound.
pub fn must_be_strictly_smaller_than_f64(
    value: f64,
    lower_bound: f64,
) -> Result<(), validation_errors::SingleFieldError> {
    if value < lower_bound {
        Ok(())
    } else {
        Err(validation_errors::SingleFieldError::MustBeSmallerThan((), lower_bound))
    }
}

#[cfg(not(feature = "pgrx"))]
#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(must_be_strictly_positive_f32(3.0_f32).is_ok());
        assert_eq!(
            must_be_strictly_positive_f32(-3.0_f32).unwrap_err(),
            validation_errors::SingleFieldError::UnexpectedNegativeOrZeroValue(())
        );
        assert_eq!(
            must_be_strictly_positive_f32(0.0_f32).unwrap_err(),
            validation_errors::SingleFieldError::UnexpectedNegativeOrZeroValue(())
        );
        assert_eq!(
            must_be_strictly_positive_f32(-0.0_f32).unwrap_err(),
            validation_errors::SingleFieldError::UnexpectedNegativeOrZeroValue(())
        );
    }

    #[test]
    fn test_must_be_strictly_greater_than_f64() {
        assert!(must_be_strictly_greater_than_f64(3.0_f64, 2.0_f64).is_ok());
        assert_eq!(
            must_be_strictly_greater_than_f64(3.0_f64, 3.0_f64).unwrap_err(),
            validation_errors::SingleFieldError::MustBeGreaterThan((), 3.0)
        );
        assert_eq!(
            must_be_strictly_greater_than_f64(3.0_f64, 4.0_f64).unwrap_err(),
            validation_errors::SingleFieldError::MustBeGreaterThan((), 4.0)
        );
    }

    #[test]
    fn test_must_be_strictly_smaller_than_f64() {
        assert!(must_be_strictly_smaller_than_f64(3.0_f64, 4.0_f64).is_ok());
        assert_eq!(
            must_be_strictly_smaller_than_f64(3.0_f64, 3.0_f64).unwrap_err(),
            validation_errors::SingleFieldError::MustBeSmallerThan((), 3.0)
        );
        assert_eq!(
            must_be_strictly_smaller_than_f64(3.0_f64, 2.0_f64).unwrap_err(),
            validation_errors::SingleFieldError::MustBeSmallerThan((), 2.0)
        );
    }
}
