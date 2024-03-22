//! Handles the swear word filter with the use of the `validator` and `rustrict` crates.
//!
//! # Disclaimer
//! Due to the nature of this genre of validation, it necessarily contains language that is not suitable for all audiences
//! as part of the test suite. If you are not comfortable with this, please do not read the test suite.
//! READER DISCRETION IS ADVISED.

use rustrict::CensorStr;
use validator::ValidationError;
use web_common_derive::custom_validator;

#[custom_validator("This field cannot contain inappropriate language")]
/// Validates that the input does not contain any slurs.
///
/// # Arguments
/// * `v` - The input to validate.
///
/// # Example
///
/// ```rust
/// use web_common::custom_validators::no_slurs;
///
/// assert!(no_slurs(&"Hello").is_ok());
/// assert!(no_slurs(&"Hello there").is_ok());
/// assert!(no_slurs(&"Hello there, friend").is_ok());
/// assert!(no_slurs(&"Hello there, shit!").is_err());
/// assert!(no_slurs(&"Hello there, ass!").is_err());
/// ```
pub fn no_slurs<S>(v: &S) -> Result<(), ValidationError>
where
    S: AsRef<str>,
{
    if v.as_ref().is_inappropriate() {
        return Err(ValidationError::new("no_slurs"));
    }
    Ok(())
}
