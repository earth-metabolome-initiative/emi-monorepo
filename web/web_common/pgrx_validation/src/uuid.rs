//! Submodule for UUID validation

use pgrx_validation_derive::validation;

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

#[cfg(not(feature = "pgrx"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_must_be_distinct_uuid() {
        let uuid1 = rosetta_uuid::Uuid::new_v4();
        let uuid2 = rosetta_uuid::Uuid::new_v4();
        assert!(must_be_distinct_uuid(uuid1, uuid2).is_ok());
        assert_eq!(
            must_be_distinct_uuid(uuid1, uuid1).unwrap_err(),
            validation_errors::DoubleFieldError::NotDistinct((), ())
        );
    }
}