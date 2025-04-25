//! Submodule defining options for the SQL generation process.

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
/// Options for SQL generation.
pub struct SQLGenerationOptions {
    /// Whether to include the population of the database in the generated SQL.
    pub(crate) include_population: bool,
}

impl SQLGenerationOptions {
    #[must_use]
    /// Sets the option to include the population of the database in the
    /// generated SQL.
    pub fn include_population(mut self) -> Self {
        self.include_population = true;
        self
    }
}
