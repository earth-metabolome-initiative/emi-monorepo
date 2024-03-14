use validator::ValidationError;
use web_common_derive::custom_validator;

#[custom_validator("No trailing spaces allowed.")]
pub fn no_trailing_spaces<S>(v: &S) -> Result<(), ValidationError>
where
    S: AsRef<str>,
{
    if v.as_ref().ends_with(' '){
        return Err(ValidationError::new("no_trailing_spaces"));
    }
    Ok(())
}