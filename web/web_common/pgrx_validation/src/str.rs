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
/// * `validation_errors::prelude::SingleFieldError::InvalidMail` if the string
///   is not a valid mail address.
pub fn must_be_email(value: &str) -> Result<(), validation_errors::prelude::SingleFieldError> {
    must_not_be_empty(value)?;
    if value.validate_email() { Ok(()) } else { Err("Invalid mail address".into()) }
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
/// * `validation_errors::prelude::SingleFieldError::EmptyText` if the string is
///   empty.
pub fn must_not_be_empty(value: &str) -> Result<(), validation_errors::prelude::SingleFieldError> {
    if value.is_empty() {
        Err(validation_errors::prelude::SingleFieldError::empty_text())
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
/// * `validation_errors::prelude::SingleFieldError::PaddedText` if the
///  string has leading or trailing whitespace.
pub fn must_not_be_padded(value: &str) -> Result<(), validation_errors::prelude::SingleFieldError> {
    if value.trim_start() != value || value.trim_end() != value {
        Err("Must not be padded".into())
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
/// * `validation_errors::prelude::SingleFieldError::ConsecutiveWhitespace` if
///   the
/// string contains two or more consecutive whitespace.
pub fn must_not_contain_consecutive_whitespace(
    value: &str,
) -> Result<(), validation_errors::prelude::SingleFieldError> {
    if value.contains("  ") {
        Err("Must not contain consecutive whitespace".into())
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
/// * `validation_errors::prelude::SingleFieldError::ControlCharacters` if the
/// string contains control characters.
pub fn must_not_contain_control_characters(
    value: &str,
) -> Result<(), validation_errors::prelude::SingleFieldError> {
    if value.chars().any(char::is_control) {
        Err("Must not contain control characters".into())
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
/// * `validation_errors::prelude::SingleFieldError::InvalidParagraph` if the
/// string is not a valid paragraph.
pub fn must_be_paragraph(value: &str) -> Result<(), validation_errors::prelude::SingleFieldError> {
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
    fn test_must_be_email() {
        assert!(must_be_email("marco.visani@unifr.ch").is_ok());
        assert_eq!(
            must_be_email("marco").unwrap_err().to_string(),
            "Field `Unspecified`: Invalid mail address"
        );
        assert_eq!(
            must_be_email("").unwrap_err().to_string(),
            "Field `Unspecified` must not be empty"
        );
    }

    #[test]
    fn test_must_not_be_padded() {
        assert!(must_not_be_padded("marco").is_ok());
        assert_eq!(
            must_not_be_padded(" marco ").unwrap_err().to_string(),
            "Field `Unspecified`: Must not be padded"
        );
    }

    #[test]
    fn test_must_not_contain_control_characters() {
        assert!(must_not_contain_control_characters("marco").is_ok());
        assert_eq!(
            must_not_contain_control_characters("marco\n").unwrap_err().to_string(),
            "Field `Unspecified`: Must not contain control characters"
        );
    }

    #[test]
    fn test_must_not_contain_consecutive_whitespace() {
        assert!(must_not_contain_consecutive_whitespace("marco visani").is_ok());
        assert_eq!(
            must_not_contain_consecutive_whitespace("marco  visani").unwrap_err().to_string(),
            "Field `Unspecified`: Must not contain consecutive whitespace"
        );
    }

    #[test]
    fn test_must_be_paragraph() {
        assert!(must_be_paragraph("marco visani").is_ok());
        assert_eq!(
            must_be_paragraph("marco  visani").unwrap_err().to_string(),
            "Field `Unspecified`: Must not contain consecutive whitespace"
        );
        assert_eq!(
            must_be_paragraph("marco\nvisani").unwrap_err().to_string(),
            "Field `Unspecified`: Must not contain control characters"
        );
        assert_eq!(
            must_be_paragraph("").unwrap_err().to_string(),
            "Field `Unspecified` must not be empty"
        );
    }
}
