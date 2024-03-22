use validator::ValidationError;
use web_common_derive::custom_validator;

#[custom_validator("No duplicated spaces allowed.")]
pub fn no_double_spaces<S>(v: &S) -> Result<(), ValidationError>
where
    S: AsRef<str>,
{
    if v.as_ref().contains("  ") {
        return Err(ValidationError::new("no_double_spaces"));
    }
    Ok(())
}
