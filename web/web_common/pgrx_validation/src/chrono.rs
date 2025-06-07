//! Submodule for chrono validation

use pgrx_validation_derive::validation;

#[validation]
/// Validates that the left [`TimestampUTC`](rosetta_timestamp::TimestampUTC) is
/// smaller than the right [`TimestampUTC`](rosetta_timestamp::TimestampUTC).
///
/// # Arguments
///
/// * `left`: the left [`TimestampUTC`](rosetta_timestamp::TimestampUTC).
/// * `right`: the right [`TimestampUTC`](rosetta_timestamp::TimestampUTC).
///
/// # Errors
///
/// * `validation_errors::DoubleFieldError::NotDistinct((), ())` if the two
///   integers are equal.
pub fn must_be_smaller_than_utc(
    left: rosetta_timestamp::TimestampUTC,
    right: rosetta_timestamp::TimestampUTC,
) -> Result<(), validation_errors::DoubleFieldError> {
    if left <= right {
        Ok(())
    } else {
        Err(validation_errors::DoubleFieldError::MustBeSmallerThan((), ()))
    }
}

#[validation]
/// Validates that the left [`TimestampUTC`](rosetta_timestamp::TimestampUTC) is
/// strictly smaller than the right
/// [`TimestampUTC`](rosetta_timestamp::TimestampUTC).
///
/// # Arguments
///
/// * `left`: the left [`TimestampUTC`](rosetta_timestamp::TimestampUTC).
/// * `right`: the right [`TimestampUTC`](rosetta_timestamp::TimestampUTC).
///
/// # Errors
///
/// * `validation_errors::DoubleFieldError::NotDistinct((), ())` if the two
///   integers are equal.
pub fn must_be_strictly_smaller_than_utc(
    left: rosetta_timestamp::TimestampUTC,
    right: rosetta_timestamp::TimestampUTC,
) -> Result<(), validation_errors::DoubleFieldError> {
    if left < right {
        Ok(())
    } else {
        Err(validation_errors::DoubleFieldError::MustBeSmallerThan((), ()))
    }
}

#[cfg(not(feature = "pgrx"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_must_be_smaller_than_utc() {
        let left = ::chrono::DateTime::<chrono::Utc>::from_timestamp(1000, 0).unwrap().into();
        let right = ::chrono::DateTime::<chrono::Utc>::from_timestamp(2000, 0).unwrap().into();
        assert!(must_be_smaller_than_utc(left, right).is_ok());
        assert!(must_be_smaller_than_utc(left, left).is_ok());
        assert!(must_be_smaller_than_utc(right, left).is_err());
    }

    #[test]
    fn test_must_be_strictly_smaller_than_utc() {
        let left = ::chrono::DateTime::<chrono::Utc>::from_timestamp(1000, 0).unwrap().into();
        let right = ::chrono::DateTime::<chrono::Utc>::from_timestamp(2000, 0).unwrap().into();
        assert!(must_be_strictly_smaller_than_utc(left, right).is_ok());
        assert!(must_be_strictly_smaller_than_utc(left, left).is_err());
        assert!(must_be_strictly_smaller_than_utc(right, left).is_err());
    }
}
