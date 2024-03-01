use validator::ValidationError;

/// A custom validator that checks if a string is not empty or None.
pub fn non_empty_or_none(value: &Option<String>) -> Result<(), ValidationError> {
    if let Some(value) = value {
        if value.is_empty() {
            return Err(ValidationError::new("Value cannot be empty."));
        }
    }
    Ok(())
}