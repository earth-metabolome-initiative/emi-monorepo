//! Submodule defining a set of translation options.

/// Trait defining translation options for the `pg2sqlite` library.
pub trait TranslationOptions {
    /// Sets the option to drop check constraints containing unsupported
    fn remove_unsupported_check_constraints(self) -> Self;

    /// Returns whether to drop check constraints containing unsupported
    /// functions.
    fn should_remove_unsupported_check_constraints(&self) -> bool;
}
