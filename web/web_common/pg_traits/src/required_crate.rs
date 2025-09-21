//! Submodule providing a struct defining the crate required by some type found
//! in the postgres database schema.

/// Struct defining the crate required by some type found in the postgres database schema.
pub struct RequiredCrate {
    /// The name of the crate.
    name: String,
    /// The version of the crate.
    version: String,
}

impl RequiredCrate {
    /// Creates a new `RequiredCrate`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the crate.
    /// * `version` - The version of the crate.
    pub fn new(name: String, version: String) -> Self {
        Self { name, version }
    }

    /// Returns the name of the crate.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the version of the crate.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
}
