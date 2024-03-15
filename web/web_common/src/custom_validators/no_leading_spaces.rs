use validator::ValidationError;
use web_common_derive::custom_validator;

#[custom_validator("No leading spaces allowed.")]
pub fn no_leading_spaces<S>(v: &S) -> Result<(), ValidationError>
where
    S: AsRef<str>,
{
    if v.as_ref().starts_with(' ') {
        return Err(ValidationError::new("no_leading_spaces"));
    }
    Ok(())
}
