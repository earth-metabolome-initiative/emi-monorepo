//! Crate to sanitize strings and convert them to snake_case.


#[derive(Debug, PartialEq, Clone)]
/// Errors that can occur during the sanitization process.
pub enum SanitizationErrors {
    /// Errors associated with the needle.
    EmptyNeedle,
    /// Errors associated with the replacement.
    EmptyReplacement,
    /// Errors associated with the needle being duplicated.
    DuplicatedNeedle(String),
    /// Term contained only underscores.
    OnlyUnderscores,
    /// Term was empty.
    EmptyTerm,
}

#[derive(Debug, PartialEq, Clone, Default)]
/// Structs defining the properties of the snake_case_sanitizer.
pub struct Sanitizer<'a> {
    /// Mapping of needles and their replacements.
    replacements: Vec<(&'a str, &'a str)>,
    /// Whether to remove leading underscores.
    remove_leading_underscores: bool,
    /// Whether to remove trailing underscores.
    remove_trailing_underscores: bool,
}

/// Default replacements.
pub const DEFAULT_REPLACEMENTS: [(&str, &str); 5] = [
    ("°C", "celsius"),
    ("°F", "fahrenheit"),
    ("°", "degrees"),
    ("%", "percent"),
    ("$", "dollar")
];

impl<'a> Sanitizer<'a> {
    /// Set whether to remove leading underscores.
    pub fn remove_leading_underscores(mut self) -> Self {
        self.remove_leading_underscores = true;
        self
    }

    /// Set whether to remove trailing underscores.
    pub fn remove_trailing_underscores(mut self) -> Self {
        self.remove_trailing_underscores = true;
        self
    }

    /// Whether to include the default replacements.
    pub fn include_defaults(mut self) -> Self {
        for (needle, replacement) in DEFAULT_REPLACEMENTS.iter() {
            self = self.add_replacement(needle, replacement).unwrap();
        }
        self
    }

    /// Add a replacement to the Sanitizer.
    pub fn add_replacement(mut self, needle: &'a str, replacement: &'a str) -> Result<Self, SanitizationErrors> {
        if needle.is_empty() {
            return Err(SanitizationErrors::EmptyNeedle);
        }
        if replacement.is_empty() {
            return Err(SanitizationErrors::EmptyReplacement);
        }

        if self.replacements.iter().any(|(n, _)| n == &needle) {
            return Err(SanitizationErrors::DuplicatedNeedle(needle.to_owned()));
        }

        self.replacements.push((needle, replacement));

        // We sort the replacements so that the largest needles are replaced first.
        self.replacements.sort_by(|(a, _), (b, _)| b.len().cmp(&a.len()));

        Ok(self)
    }

    /// Applies replacements to the provided string.
    fn apply_replacement<S: ToString>(&self, input: S) -> String {
        let mut changed = true;
        let mut target = input.to_string();
        while changed {
            changed = false;
            for (needle, replacement) in &self.replacements {
                if target.contains(needle) {
                    changed = true;
                    target = target.replace(needle, replacement);
                    break;
                }
            }
        }
        target
    }

    /// Sanitize the provided string.
    /// 
    /// # Arguments
    /// 
    /// * `input` - The string to sanitize.
    /// 
    /// # Returns
    /// 
    /// The sanitized string.
    /// 
    /// # Errors
    /// 
    /// * `SanitizationErrors::EmptyTerm` - The term was empty.
    /// * `SanitizationErrors::OnlyUnderscores` - The term contained only underscores.
    /// 
    pub fn to_snake_case<S: ToString>(&self, input: S) -> Result<String, SanitizationErrors> {
        let mut with_replacements = self.apply_replacement(input);

        if with_replacements.is_empty() {
            return Err(SanitizationErrors::EmptyTerm);
        }

        if self.remove_leading_underscores {
            with_replacements = with_replacements.trim_start_matches('_').to_owned();
        }
        if self.remove_trailing_underscores {
            with_replacements = with_replacements.trim_end_matches('_').to_owned();
        }

        if with_replacements.is_empty() {
            return Err(SanitizationErrors::OnlyUnderscores);
        }

        let mut result = String::new();
        let mut last_was_underscore = false;
        let mut last_was_uppercase = false;

      for mut character in with_replacements.chars() {
            if !character.is_ascii_alphanumeric() {
                character = '_';
            }

            if character == '_' {
                if last_was_underscore {
                    continue;
                }
                if result.is_empty() && self.remove_leading_underscores {
                    continue;
                }
                last_was_underscore = true;
                result.push('_');
                continue;
            }

            if character.is_ascii_uppercase() {
                if !result.is_empty() && !last_was_underscore && !last_was_uppercase {
                    result.push('_');
                }
                result.push(character.to_ascii_lowercase());
                last_was_uppercase = true;
                continue;
            }

            result.push(character);
            last_was_underscore = false;
            last_was_uppercase = false;
        }

        if self.remove_trailing_underscores {
            result = result.trim_end_matches('_').to_owned();
        }

        Ok(result)
    }

    /// Returns the string sanitized to CamelCase.
    pub fn to_camel_case<S: ToString>(&self, input: S) -> Result<String, SanitizationErrors> {
        Ok(self.to_snake_case(input)?.split('_').map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
            }
        }).collect())
    }
}