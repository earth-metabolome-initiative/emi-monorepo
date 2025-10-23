//! Submodule defining a struct which represents a rust module.

mod builder;

pub use builder::InternalTraitBuilder;
use syn::Ident;

use crate::structs::{InternalCrate, InternalToken, Publicness};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a rust trait.
pub struct InternalTrait<'data> {
    /// Name of the trait.
    name: String,
    /// Publicness of the trait.
    publicness: Publicness,
    /// Internal token streams defined within the trait.
    internal_tokens: Vec<InternalToken<'data>>,
    /// Trait documentation.
    documentation: String,
}

impl<'data> InternalTrait<'data> {
    /// Initializes a new `InternalTraitBuilder`.
    pub fn new() -> InternalTraitBuilder<'data> {
        InternalTraitBuilder::default()
    }

    /// Returns the name of the module.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the ident of the module.
    pub fn ident(&self) -> Ident {
        syn::Ident::new(&self.name, proc_macro2::Span::call_site())
    }

    /// Returns the publicness of the module.
    pub fn publicness(&self) -> &Publicness {
        &self.publicness
    }

    /// Returns whether the module is public.
    pub fn is_public(&self) -> bool {
        self.publicness.is_public()
    }

    /// Returns the sorted unique internal dependencies of the module.
    pub fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        let mut dependencies = Vec::new();
        for token in &self.internal_tokens {
            dependencies.extend(token.internal_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }

    /// Returns the sorted unique external dependencies of the module.
    pub fn external_dependencies(&self) -> Vec<&crate::structs::ExternalCrate<'data>> {
        let mut dependencies = Vec::new();
        for token in &self.internal_tokens {
            dependencies.extend(token.external_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}
