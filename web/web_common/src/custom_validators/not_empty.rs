use validator::ValidationError;
use web_common_derive::custom_validator;

#[custom_validator("This field cannot be empty.")]
pub fn not_empty<S>(v: &S) -> Result<(), ValidationError>
where
    S: AsRef<str>,
{
    if v.as_ref().is_empty() {
        return Err(ValidationError::new("not_empty"));
    }
    Ok(())
}