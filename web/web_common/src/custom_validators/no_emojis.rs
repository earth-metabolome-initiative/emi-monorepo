//! Submodule providing a validator that checks if a string contains any emojis.

use validator::ValidationError;
use web_common_derive::custom_validator;

pub fn is_emoji(c: char) -> bool {
    // Emojis are defined by a specific codepoint range
    (c >= '\u{1F600}') && (c <= '\u{1F64F}') ||  // Emoticons block
       (c >= '\u{1F680}') && (c <= '\u{1F6C5}') ||  // Emoticons block
       (c >= '\u{2702}') && (c <= '\u{27B0}') ||  // Dingbats block
       (c >= '\u{1F1E6}') && (c <= '\u{1F1FF}') // Flags block
}

#[custom_validator("This field cannot contain emojis")]
/// Validates that the input does not contain any emojis.
///
/// # Example
///
/// ```rust
/// use web_common::custom_validators::no_emojis;
///
/// assert!(no_emojis(&"Hello").is_ok());
/// assert!(no_emojis(&"Hello World").is_ok());
/// assert!(no_emojis(&"Hello World!").is_ok());
/// assert!(no_emojis(&"Hello World?").is_ok());
/// assert!(no_emojis(&"Hello World*").is_ok());
/// assert!(no_emojis(&"Pierre Marie 😍").is_err());
/// assert!(no_emojis(&"Pierre-Marie").is_ok());
/// assert!(no_emojis(&"Pierre_Marie 🇮🇹").is_err());
/// assert!(no_emojis(&"Pierre-Marie Jr. ✓").is_err());
/// ```
pub fn no_emojis<S>(v: &S) -> Result<(), ValidationError>
where
    S: AsRef<str>,
{
    if v.as_ref().chars().any(is_emoji) {
        return Err(ValidationError::new("This field cannot contain emojis"));
    }

    Ok(())
}
