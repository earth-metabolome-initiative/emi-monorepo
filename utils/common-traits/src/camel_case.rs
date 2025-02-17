//! Converts a string to camel case.

/// Converts a string to camel case.
pub trait ToCamelCase {
    /// Converts a string to camel case.
    fn to_camel_case(&self) -> String;
}

impl<T: ToString> ToCamelCase for T {
    fn to_camel_case(&self) -> String {
        self.to_string()
            .split('_')
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_camel_case() {
        assert_eq!("HelloWorld".to_string(), "hello_world".to_camel_case());
        assert_eq!("HelloWorld".to_string(), "_hello_world".to_string().to_camel_case());
        assert_eq!("HelloWorld".to_string(), "hello_world_".to_string().to_camel_case());
        assert_eq!("HelloWorld".to_string(), "_hello_world_".to_string().to_camel_case());
    }
}
