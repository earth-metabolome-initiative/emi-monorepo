//! Submodule providing a struct defining a trait available in an external
//! crate.

mod builder;
use std::{fmt::Debug, hash::Hash};

use builder::ExternalTraitBuilder;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct defining a trait available in an external crate.
pub struct ExternalTrait {
    /// The name of the trait.
    name: String,
}

unsafe impl Send for ExternalTrait {}
unsafe impl Sync for ExternalTrait {}

impl ExternalTrait {
    /// Inizializes a new `ExternalTraitBuilder`.
    pub fn new() -> ExternalTraitBuilder {
        ExternalTraitBuilder::default()
    }

    /// Returns the name of the trait.
    pub fn name(&self) -> &str {
        &self.name
    }
}
