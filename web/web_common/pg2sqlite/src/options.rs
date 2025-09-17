//! Submodule defining a struct providing options for the translation.

use crate::traits::TranslationOptions;

/// Struct to hold options for the translation.
#[derive(Debug, Clone, Default)]
pub struct Pg2SqliteOptions {
    /// Whether to drop check constraints containing unsupported functions.
    pub remove_unsupported_check_constraints: bool,
}

impl TranslationOptions for Pg2SqliteOptions {
    fn remove_unsupported_check_constraints(mut self) -> Self {
        self.remove_unsupported_check_constraints = true;
        self
    }

    fn should_remove_unsupported_check_constraints(&self) -> bool {
        self.remove_unsupported_check_constraints
    }
}
