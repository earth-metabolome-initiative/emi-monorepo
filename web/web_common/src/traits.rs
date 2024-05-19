//! Utiliti traits for formatting and matching strings.

pub trait CapitalizeString {
    /// Returns the provided string with the first letter capitalized.
    fn capitalize(&self) -> String;
}

impl<S: AsRef<str>> CapitalizeString for S {
    fn capitalize(&self) -> String {
        let mut chars = self.as_ref().chars();
        match chars.next() {
            None => String::new(),
            Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}