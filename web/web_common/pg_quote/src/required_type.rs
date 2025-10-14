//! Submodule providing a struct defining the type required by some type found
//! in the postgres database schema.

use crate::required_type::builder::RequiredTypeBuilder;

mod builder;
mod traits_mask;
pub use traits_mask::Trait;

#[derive(Clone)]
/// Struct defining the type required by some type found in the postgres
/// database schema.
pub struct RequiredType {
    /// The diesel type defined within the crate compatible with the given
    /// postgres type.
    diesel_type: syn::Type,
    /// The rust type defined within the crate compatible with the given
    /// postgres type.
    rust_type: syn::Type,
    /// The postgres types which are compatible with the diesel and rust types
    /// defined within the crate.
    postgres_types: Vec<&'static str>,
    /// The traits supported by the current type.
    traits: traits_mask::TraitsMask,
}

impl RequiredType {
    /// Inizializes a new `RequiredTypeBuilder`.
    pub fn new() -> RequiredTypeBuilder {
        RequiredTypeBuilder::default()
    }

    /// Returns the diesel type defined within the crate compatible with the
    /// given postgres type.
    pub fn diesel_type(&self) -> &syn::Type {
        &self.diesel_type
    }

    /// Returns the rust type defined within the crate compatible with the given
    /// postgres type.
    pub fn rust_type(&self) -> &syn::Type {
        &self.rust_type
    }

    /// Returns a reference over the postgres types which are compatible with
    /// the diesel and rust types defined within the crate.
    pub fn postgres_types(&self) -> &[&'static str] {
        &self.postgres_types
    }

    /// Returns whether the Rust type associated with the current `RequiredType`
    /// supports the given trait.
    ///
    /// # Arguments
    ///
    /// * `trait` - The trait to check support for.
    pub fn supports(&self, r#trait: Trait) -> bool {
        self.traits.supports(r#trait)
    }

    /// Returns whether the current `RequiredType` is compatible with the given
    /// postgres type.
    ///
    /// # Arguments
    /// * `postgres_type` - The postgres type to check compatibility with.
    pub fn is_compatible_with(&self, postgres_type: &str) -> bool {
        self.postgres_types.contains(&postgres_type)
    }
}
