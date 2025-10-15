//! Submodule providing a struct defining the crate required by some type found
//! in the postgres database schema.

use crate::{RequiredType, required_crate::builder::RequiredCrateBuilder};

mod builder;
mod core_crate;
mod postgis_diesel_crate;
mod std_crate;

/// Struct defining the crate required by some type found in the postgres
/// database schema.
pub struct RequiredCrate {
    /// The name of the crate.
    name: String,
    /// List of postgres types and their corresponding diesel and rust types
    /// defined within the crate.
    types: Vec<RequiredType>,
}

impl RequiredCrate {
    /// Inizializes a new `RequiredCrateBuilder`.
    pub fn new() -> RequiredCrateBuilder {
        RequiredCrateBuilder::default()
    }

    /// Returns a reference to the name of the crate.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the name of the crate.
    ///
    /// # Arguments
    ///
    /// * `postgres_type` - The postgres type to find a compatible type for.
    pub fn compatible_type(&self, postgres_type: &str) -> Option<&RequiredType> {
        self.types.iter().find(|ty| ty.is_compatible_with(postgres_type))
    }
}
