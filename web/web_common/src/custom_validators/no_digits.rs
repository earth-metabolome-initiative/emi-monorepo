use validator::ValidationError;
use web_common_derive::custom_validator;

#[custom_validator("This field cannot contain digits")]
/// Validates that the input does not contain any digits.
/// 
/// # Arguments
/// * `v` - The input to validate.
///
/// # Example
///
/// ```rust
/// use web_common::custom_validators::no_digits;
///
/// assert!(no_digits(&"John").is_ok());
/// assert!(no_digits(&"John Doe").is_ok());
/// assert!(no_digits(&"John Doe Jr.").is_ok());
/// assert!(no_digits(&"John Doe Jr. III").is_ok());
///
/// assert!(no_digits(&"John 3rd").is_err());
/// assert!(no_digits(&"John Doe Jr. III 3rd").is_err());
/// assert!(no_digits(&"Jo3hn Doe Jr. III Esq. PhD.").is_err());
/// ```
pub fn no_digits<S>(v: &S) -> Result<(), ValidationError>
where
    S: AsRef<str>,
{
    if v.as_ref().chars().any(|c| c.is_numeric()) {
        return Err(ValidationError::new("no_digits"));
    }
    Ok(())
}
