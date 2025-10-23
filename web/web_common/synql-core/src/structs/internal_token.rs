//! Submodule defining a struct which represents a tokenstream alongside its
//! metadata.

mod builder;

use std::hash::Hash;

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

impl PartialOrd for InternalToken<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for InternalToken<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let stream_cmp = self.stream.to_string().cmp(&other.stream.to_string());
        if stream_cmp != std::cmp::Ordering::Equal {
            return stream_cmp;
        }

        let publicness_cmp = self.publicness.cmp(&other.publicness);
        if publicness_cmp != std::cmp::Ordering::Equal {
            return publicness_cmp;
        }

        let external_macros_cmp = self.external_macros.cmp(&other.external_macros);
        if external_macros_cmp != std::cmp::Ordering::Equal {
            return external_macros_cmp;
        }

        let external_traits_cmp = self.external_traits.cmp(&other.external_traits);
        if external_traits_cmp != std::cmp::Ordering::Equal {
            return external_traits_cmp;
        }

        let internal_data_cmp = self.internal_data.cmp(&other.internal_data);
        if internal_data_cmp != std::cmp::Ordering::Equal {
            return internal_data_cmp;
        }

        self.internal_modules.cmp(&other.internal_modules)
    }
}

impl Hash for InternalToken<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.stream.to_string().hash(state);
        self.publicness.hash(state);
        self.external_macros.hash(state);
        self.external_traits.hash(state);
        self.internal_data.hash(state);
        self.internal_modules.hash(state);
    }
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
    pub fn external_dependencies(&self) -> Vec<&ExternalCrate<'data>> {
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
