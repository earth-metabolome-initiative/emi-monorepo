//! Submodule for string validation

use pgrx_validation_derive::validation;
use validator::ValidateEmail;

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
    if value.validate_email() {
        Ok(())
    } else {
        Err(validation_errors::SingleFieldError::InvalidMail(()))
    }
}

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
    if value.is_empty() { Err(validation_errors::SingleFieldError::EmptyText(())) } else { Ok(()) }
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
/// Validates that a string has no leading or trailing whitespace.
///
/// # Arguments
///
/// * `value` a string
///
/// # Errors
///
/// * `validation_errors::SingleFieldError::PaddedText` if the
///  string has leading or trailing whitespace.
pub fn must_not_be_padded(value: &str) -> Result<(), validation_errors::SingleFieldError> {
    if value.trim_start() != value || value.trim_end() != value {
        Err(validation_errors::SingleFieldError::PaddedText(()))
    } else {
        Ok(())
    }
}

#[validation]
/// Validates that a string does not contain two or more consecutive whitespace
///
/// # Arguments
///
/// * `value` a string
///
/// # Errors
///
/// * `validation_errors::SingleFieldError::ConsecutiveWhitespace` if the
/// string contains two or more consecutive whitespace.
pub fn must_not_contain_consecutive_whitespace(
    value: &str,
) -> Result<(), validation_errors::SingleFieldError> {
    if value.contains("  ") {
        Err(validation_errors::SingleFieldError::ConsecutiveWhitespace(()))
    } else {
        Ok(())
    }
}

#[validation]
/// Validates that a string does not contains any control characters.
///
/// # Arguments
///
/// * `value` a string
///
/// # Errors
///
/// * `validation_errors::SingleFieldError::ControlCharacters` if the
/// string contains control characters.
pub fn must_not_contain_control_characters(
    value: &str,
) -> Result<(), validation_errors::SingleFieldError> {
    if value.chars().any(char::is_control) {
        Err(validation_errors::SingleFieldError::ControlCharacters(()))
    } else {
        Ok(())
    }
}

#[validation]
/// Validates that a string is a valid paragraph.
/// It calls:
/// * `must_not_be_empty`
/// * `must_not_be_padded`
/// * `must_not_contain_consecutive_whitespace`
/// * `must_not_contain_control_characters`
///
/// # Arguments
///
/// * `value` a string
///
/// # Errors
///
/// * `validation_errors::SingleFieldError::InvalidParagraph` if the
/// string is not a valid paragraph.
pub fn must_be_paragraph(value: &str) -> Result<(), validation_errors::SingleFieldError> {
    must_not_be_empty(value)?;
    must_not_be_padded(value)?;
    must_not_contain_consecutive_whitespace(value)?;
    must_not_contain_control_characters(value)?;
    Ok(())
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
    fn test_must_not_be_padded() {
        assert!(must_not_be_padded("marco").is_ok());
        assert_eq!(
            must_not_be_padded(" marco ").unwrap_err(),
            validation_errors::SingleFieldError::PaddedText(())
        );
    }

    #[test]
    fn test_must_not_contain_control_characters() {
        assert!(must_not_contain_control_characters("marco").is_ok());
        assert_eq!(
            must_not_contain_control_characters("marco\n").unwrap_err(),
            validation_errors::SingleFieldError::ControlCharacters(())
        );
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
    fn test_must_not_contain_consecutive_whitespace() {
        assert_eq!(must_not_contain_consecutive_whitespace("marco visani"), Ok(()));
        assert_eq!(
            must_not_contain_consecutive_whitespace("marco  visani").unwrap_err(),
            validation_errors::SingleFieldError::ConsecutiveWhitespace(())
        );
    }

    #[test]
    fn test_must_be_paragraph() {
        assert_eq!(must_be_paragraph("marco visani"), Ok(()));
        assert_eq!(
            must_be_paragraph("marco  visani").unwrap_err(),
            validation_errors::SingleFieldError::ConsecutiveWhitespace(())
        );
        assert_eq!(
            must_be_paragraph("marco\nvisani").unwrap_err(),
            validation_errors::SingleFieldError::ControlCharacters(())
        );
        assert_eq!(
            must_be_paragraph("").unwrap_err(),
            validation_errors::SingleFieldError::EmptyText(())
        );
    }
}
