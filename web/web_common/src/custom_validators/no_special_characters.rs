use super::no_emojis;
use validator::ValidationError;
use web_common_derive::custom_validator;

pub const INVALID_NAME_CHARS: [char; 172] = [
    '\x00', '\x01', '\x02', '\x03', '\x04', '\x05', '\x06', '\x07', '\x08', '\t', '\n', '\x0b',
    '\x0c', '\r', '\x0e', '\x0f', '\x10', '\x11', '\x12', '\x13', '\x14', '\x15', '\x16', '\x17',
    '\x18', '\x19', '\x1a', '\x1b', '\x1c', '\x1d', '\x1e', '\x1f',
    '\x7f', // ASCII control characters
    // Mathematical symbols and other special characters
    '*', '+', '/', '=', '<', '>', '?', '@', '[', '\\', ']', '^', '{', '}', '|', '~', '%', '&', '!',
    '"', '#', '$', '(', ')', ',', ':', ';', '¡', '¢', '£', '¤', '¥', '¦', '§', '¨', '©', 'ª', '«',
    '¬', '®', '¯', '°', '±', '²', '³', '\u{00b4}', '·', '\u{00b8}', '¹', 'º', '»', '¼', '½', '¾',
    '¿', '\u{00d7}', '÷', '_', '`', '\u{02c6}', '˘', '˙', '˚',
    // Additional characters
    '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}',
    '\u{2008}', '\u{2009}', '\u{200a}', '\u{200b}', '\u{200c}', '\u{200d}', '\u{200e}', '\u{200f}',
    '\u{2028}', '\u{2029}', '\u{202a}', '\u{202b}', '\u{202c}', '\u{202d}', '\u{202e}', '\u{202f}',
    '\u{205f}', '\u{2060}', '\u{2061}', '\u{2062}', '\u{2063}', '\u{2064}', '\u{2065}', '\u{2066}',
    '\u{2067}', '\u{2068}', '\u{2069}', '\u{206a}', '\u{206b}', '\u{206c}', '\u{206d}', '\u{206e}',
    '\u{206f}', '\u{feff}', '\u{fff9}', '\u{fffa}', '\u{fffb}', '\u{fffc}', '\u{fffd}', '\u{fffe}',
    '\u{ffff}', // Extended characters
    '\u{fe4d}', '\u{fe4e}', '\u{fe4f}', '﹑', '﹔', '﹕', '﹖', '﹗', '\u{fe58}', '﹙', '﹚', '﹛',
    '﹜', '﹝', '﹞', '﹟', '﹠', '﹡', '﹢', '﹣', '﹤', '﹥', '﹦', '\u{fe68}', '﹩', '﹪', '﹫',
];

pub const INVALID_NAME_REPEATED_CHARS: [char; 5] = ['-', '.', ',', '\'', ' '];

pub const INVALID_LEADING_CHARS: [char; 5] = ['-', '.', ',', '\'', ' '];

pub const INVALID_TRAILING_CHARS: [char; 3] = ['-', ',', ' '];

#[custom_validator("This field cannot contain special characters")]
/// Validates that the input does not contain any special_characters.
///
/// # Example
///
/// ```rust
/// use web_common::custom_validators::no_special_characters;
///
/// assert!(no_special_characters(&"Hello").is_ok());
/// assert!(no_special_characters(&"Hello World").is_ok());
/// assert!(no_special_characters(&"Hello World!").is_err());
/// assert!(no_special_characters(&"Hello World?").is_err());
/// assert!(no_special_characters(&"Hello World*").is_err());
/// assert!(no_special_characters(&"Pierre Marie").is_ok());
/// assert!(no_special_characters(&"Pierre-Marie").is_ok());
/// assert!(no_special_characters(&"Pierre_Marie").is_err());
/// assert!(no_special_characters(&"Pierre-Marie Jr.").is_ok());
/// assert!(no_special_characters(&"Pierre-Marie Jr. ✓").is_err());
/// assert!(no_special_characters(&"Pierre--Marie Jr.").is_err());
/// assert!(no_special_characters(&"Pierre-Marie Jr..").is_err());
/// assert!(no_special_characters(&"O'Connell").is_ok());
/// assert!(no_special_characters(&"O''Connell").is_err());
/// assert!(no_special_characters(&"O'''Connell").is_err());
/// assert!(no_special_characters(&"O''''Connell").is_err());
/// assert!(no_special_characters(&"O'_'_Connell").is_err());
/// assert!(no_special_characters(&"O'_Connell").is_err());
/// assert!(no_special_characters(&"O'Connell Jr.").is_ok());
/// assert!(no_special_characters(&"O'Connell Jr..").is_err());
/// assert!(no_special_characters(&"O'Connell Jr.-").is_err());
/// assert!(no_special_characters(&"O'Connell Jr-").is_err());
/// assert!(no_special_characters(&"-O'Connell Jr").is_err());
/// assert!(no_special_characters(&"O'Connell Jr'").is_ok());
/// ```
///  
/// The const array `INVALID_NAME_CHARS` contains all the characters that are considered special characters.
/// You can import it and use it in your own custom validators with:
///
/// ```rust
/// use web_common::custom_validators::INVALID_NAME_CHARS;
///
/// assert_eq!(
///     INVALID_NAME_CHARS.len(),
///     INVALID_NAME_CHARS
///         .iter()
///         .collect::<std::collections::HashSet<_>>()
///         .len(),
///     "The following characters appear multiple times in the `INVALID_NAME_CHARS` array: {:?}",
///     INVALID_NAME_CHARS
///         .iter()
///         .filter(|c| INVALID_NAME_CHARS.iter().filter(|c2| c == c2).count() > 1)
///         .collect::<std::collections::HashSet<_>>(),
/// );
/// ```
///
/// Additionally, the const array `INVALID_NAME_REPEATED_CHARS` contains all the characters that are considered special characters when repeated.
///
/// ```rust
/// use web_common::custom_validators::INVALID_NAME_REPEATED_CHARS;
///
/// assert_eq!(
///    INVALID_NAME_REPEATED_CHARS.len(),
///    INVALID_NAME_REPEATED_CHARS.iter().collect::<std::collections::HashSet<_>>().len(),
///    "The following characters appear multiple times in the `INVALID_NAME_REPEATED_CHARS` array: {:?}",
///    INVALID_NAME_REPEATED_CHARS.iter().filter(|c| INVALID_NAME_REPEATED_CHARS.iter().filter(|c2| c == c2).count() > 1).collect::<std::collections::HashSet<_>>(),
/// );
/// ```
///
/// Finally, the const arrays `INVALID_LEADING_CHARS` and `INVALID_TRAILING_CHARS` contain all the characters that are considered special characters when leading or trailing.
///
/// ```rust
/// use web_common::custom_validators::{INVALID_LEADING_CHARS, INVALID_TRAILING_CHARS};
///
/// assert_eq!(
///    INVALID_LEADING_CHARS.len(),
///    INVALID_LEADING_CHARS.iter().collect::<std::collections::HashSet<_>>().len(),
///    "The following characters appear multiple times in the `INVALID_LEADING_CHARS` array: {:?}",
///    INVALID_LEADING_CHARS.iter().filter(|c| INVALID_LEADING_CHARS.iter().filter(|c2| c == c2).count() > 1).collect::<std::collections::HashSet<_>>(),
/// );
///
/// assert_eq!(
///    INVALID_TRAILING_CHARS.len(),
///    INVALID_TRAILING_CHARS.iter().collect::<std::collections::HashSet<_>>().len(),
///    "The following characters appear multiple times in the `INVALID_TRAILING_CHARS` array: {:?}",
///    INVALID_TRAILING_CHARS.iter().filter(|c| INVALID_TRAILING_CHARS.iter().filter(|c2| c == c2).count() > 1).collect::<std::collections::HashSet<_>>(),
/// );
/// ```
pub fn no_special_characters<S>(v: &S) -> Result<(), ValidationError>
where
    S: AsRef<str>,
{
    if v.as_ref().chars().any(|c| INVALID_NAME_CHARS.contains(&c)) {
        return Err(ValidationError::new(
            "This field cannot contain special characters",
        ));
    }

    // Emojis are considered special characters
    no_emojis(v)?;

    // We check that the first character is not in the `INVALID_LEADING_CHARS` array
    if v.as_ref().chars().next().map_or(false, |c| INVALID_LEADING_CHARS.contains(&c)) {
        return Err(ValidationError::new(
            "This field cannot start with special characters",
        ));
    }

    // We check that the last character is not in the `INVALID_TRAILING_CHARS` array
    if v.as_ref().chars().last().map_or(false, |c| INVALID_TRAILING_CHARS.contains(&c)) {
        return Err(ValidationError::new(
            "This field cannot end with special characters",
        ));
    }

    // We check that no two subsequent characters are in the `INVALID_NAME_REPEATED_CHARS` array
    if v.as_ref()
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .any(|w| {
            INVALID_NAME_REPEATED_CHARS.contains(&w[0])
                && INVALID_NAME_REPEATED_CHARS.contains(&w[1])
        })
    {
        return Err(ValidationError::new(
            "This field cannot contain repeated symbols",
        ));
    }

    Ok(())
}
