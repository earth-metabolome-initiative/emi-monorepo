//! Submodule for integer validation

use pgrx_validation_derive::validation;

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
/// Control that the i32 is strictly positive (0, ...].
///
/// # Arguments
///
/// * `value` a i32
///
/// # Errors
///
/// * `validation_errors::SingleFieldError::UnexpectedNegativeOrZeroValue(())`
///   if the value is negative or zero.
pub fn must_be_strictly_positive_i32(
    value: i32,
) -> Result<(), validation_errors::SingleFieldError> {
    if value > 0 {
        Ok(())
    } else {
        Err(validation_errors::SingleFieldError::UnexpectedNegativeOrZeroValue(()))
    }
}

#[cfg(not(feature = "pgrx"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_must_be_distinct_i32() {
        assert!(must_be_distinct_i32(3, 4).is_ok());
        assert_eq!(
            must_be_distinct_i32(3, 3).unwrap_err(),
            validation_errors::DoubleFieldError::NotDistinct((), ())
        );
    }
}
