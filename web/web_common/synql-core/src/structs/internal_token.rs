//! Submodule defining a struct which represents a tokenstream alongside its
//! metadata.

mod builder;

pub use builder::InternalTokenBuilder;
use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::structs::{
    ExternalCrate, InternalCrate, Publicness,
    external_crate::{ExternalMacroRef, ExternalTraitRef},
    internal_data::{InternalDataRef, InternalModuleRef},
};

#[derive(Debug, Clone)]
/// Struct representing a rust tokenstream alongside its metadata.
pub struct InternalToken<'data> {
    /// Publicness of the token stream.
    publicness: Publicness,
    /// The token stream.
    stream: TokenStream,
    /// External macros used in the token stream.
    external_macros: Vec<ExternalMacroRef<'data>>,
    /// External traits used in the token stream.
    external_traits: Vec<ExternalTraitRef<'data>>,
    /// Internal data used in the token stream.
    internal_data: Vec<InternalDataRef<'data>>,
    /// Internal modules from other crates in the same workspace which are used
    /// in the token stream.
    internal_modules: Vec<InternalModuleRef<'data>>,
}

impl PartialEq for InternalToken<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.stream.to_string() == other.stream.to_string()
            && self.publicness == other.publicness
            && self.external_macros == other.external_macros
            && self.external_traits == other.external_traits
            && self.internal_data == other.internal_data
            && self.internal_modules == other.internal_modules
    }
}

impl Eq for InternalToken<'_> {}

impl<'data> InternalToken<'data> {
    /// Initializes a new `InternalTokenBuilder`.
    pub fn new() -> InternalTokenBuilder<'data> {
        InternalTokenBuilder::default()
    }

    /// Returns whether the token stream is public.
    pub fn is_public(&self) -> bool {
        self.publicness.is_public()
    }

    /// Returns the sorted unique external dependencies of the token stream.
    pub fn external_dependencies(&self) -> Vec<&'data ExternalCrate> {
        let mut dependencies = Vec::new();
        for ext_macro in &self.external_macros {
            dependencies.push(ext_macro.external_crate());
        }
        for ext_trait in &self.external_traits {
            dependencies.push(ext_trait.external_crate());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }

    /// Returns the sorted unique internal dependencies of the token stream.
    pub fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        let mut dependencies = Vec::new();
        for data in &self.internal_data {
            dependencies.extend(data.internal_dependencies());
        }
        for module in &self.internal_modules {
            dependencies.push(module.internal_crate());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}

impl ToTokens for InternalToken<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.stream.to_tokens(tokens);
    }
}
