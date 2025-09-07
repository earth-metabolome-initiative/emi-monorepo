//! Submodule defining a schema for the translation between `PostgreSQL` and
//! `SQLite`.

use sqlparser::ast::CreateFunction;

/// Trait to define a schema for the translation between `PostgreSQL` and
/// `SQLite`.
pub trait Schema {
    /// Returns a reference to a function defined in the schema by its name, if
    /// it exists.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function to be searched.
    fn has_function(&self, name: &str) -> Option<&CreateFunction>;

    /// Adds a function to the schema.
    ///
    /// # Arguments
    ///
    /// * `function` - The function to be added.
    fn add_function(&mut self, function: &CreateFunction);
}
