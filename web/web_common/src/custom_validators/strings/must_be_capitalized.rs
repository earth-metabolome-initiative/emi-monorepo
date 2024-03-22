use validator::ValidationError;
use web_common_derive::custom_validator;

#[custom_validator("This field must be properly capitalized")]
/// Validates that the input is capitalized.
///
/// # Example
/// This validation tool may be used to ensure that a user's name is properly capitalized.
/// Of course, with people's names, there are many exceptions to the rule. Let's start with
/// a few examples of using this validation tool:
///
/// ```rust
/// use web_common::custom_validators::must_be_capitalized;
///
/// assert!(must_be_capitalized(&"John").is_ok());
/// assert!(must_be_capitalized(&"John Doe").is_ok());
/// assert!(must_be_capitalized(&"John Doe Jr.").is_ok());
/// assert!(must_be_capitalized(&"John Doe Jr. III").is_ok());
/// assert!(must_be_capitalized(&"John doe Jr. III").is_err());
/// assert!(must_be_capitalized(&"John Doe jr. III").is_err());
/// assert!(must_be_capitalized(&"John Doe Jr. III Esq.").is_ok());
/// assert!(must_be_capitalized(&"John Doe Jr. III Esq. PhD.").is_ok());
/// assert!(must_be_capitalized(&"Pierre").is_ok());
/// assert!(must_be_capitalized(&"Pierre-Marie").is_ok());
/// assert!(must_be_capitalized(&"Pierre-Marie LeBlanc").is_ok());
/// assert!(must_be_capitalized(&"pierre-Marie LeBlanc III").is_err());
/// ```
///
/// The general rule, we observe, is that the first letter of each word should be capitalized.
/// The words may be split by spaces or hyphens.
pub fn must_be_capitalized<S>(v: &S) -> Result<(), ValidationError>
where
    S: AsRef<str>,
{
    // We split the words around the spaces and hyphens.
    for word in v.as_ref().split(|c: char| c == ' ' || c == '-') {
        // We check that the first letter of each word is capitalized.
        if !word
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(true)
        {
            return Err(ValidationError::new("must_be_capitalized"));
        }
    }

    Ok(())
}
