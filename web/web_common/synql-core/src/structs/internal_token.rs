//! Submodule defining a struct which represents a tokenstream alongside its
//! metadata.

mod builder;

pub use builder::InternalTokenBuilder;
use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::structs::{
    ExternalCrate, Publicness,
    external_crate::{ExternalMacroRef, ExternalTraitRef},
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
}

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
}

impl ToTokens for InternalToken<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.stream.to_tokens(tokens);
    }
}
