//! Submodule defining a set of translation options.

/// Trait defining translation options for the `pg2sqlite` library.
pub trait TranslationOptions {
    #[must_use]
    /// Sets the option to drop check constraints containing unsupported
    fn remove_unsupported_check_constraints(self) -> Self;

    /// Returns whether to drop check constraints containing unsupported
    /// functions.
    fn should_remove_unsupported_check_constraints(&self) -> bool;

    #[must_use]
    /// Sets whether should drop `CREATE INDEX` statements which do not include
    /// tables characterized by a `UUID` primary key column.
    fn drop_indexes_without_uuid_pk_tables(self) -> Self;

    /// Returns whether should drop `CREATE INDEX` statements which do not
    /// include tables characterized by a `UUID` primary key column.
    fn should_drop_indexes_without_uuid_pk_tables(&self) -> bool;
}
