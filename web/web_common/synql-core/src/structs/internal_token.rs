//! Submodule defining a struct which represents a tokenstream alongside its
//! metadata.

mod builder;

use std::hash::Hash;

pub use builder::InternalTokenBuilder;
use proc_macro2::TokenStream;
use quote::ToTokens;

mod from_ident;
mod trait_impl;

use crate::{
    structs::{
        DataVariantRef, ExternalCrate, InternalCrate, Publicness, external_crate::ExternalMacroRef,
        external_trait::TraitVariantRef, internal_data::InternalModuleRef,
        internal_token::trait_impl::TraitImpl,
    },
    traits::{ExternalDependencies, InternalDependencies},
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
    /// Traits used in the token stream.
    employed_traits: Vec<TraitVariantRef<'data>>,
    /// Traits which are implemented by the token stream.
    implemented_traits: Vec<TraitVariantRef<'data>>,
    /// Data used in the token stream.
    data: Vec<DataVariantRef<'data>>,
    /// Internal modules from other crates in the same workspace which are used
    /// in the token stream.
    internal_modules: Vec<InternalModuleRef<'data>>,
}

impl PartialEq for InternalToken<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.stream.to_string() == other.stream.to_string()
            && self.publicness == other.publicness
            && self.external_macros == other.external_macros
            && self.employed_traits == other.employed_traits
            && self.implemented_traits == other.implemented_traits
            && self.data == other.data
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

        let employed_traits_cmp = self.employed_traits.cmp(&other.employed_traits);
        if employed_traits_cmp != std::cmp::Ordering::Equal {
            return employed_traits_cmp;
        }

        let implemented_traits_cmp = self.implemented_traits.cmp(&other.implemented_traits);
        if implemented_traits_cmp != std::cmp::Ordering::Equal {
            return implemented_traits_cmp;
        }

        let internal_data_cmp = self.data.cmp(&other.data);
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
        self.employed_traits.hash(state);
        self.implemented_traits.hash(state);
        self.data.hash(state);
        self.internal_modules.hash(state);
    }
}

impl<'data> InternalToken<'data> {
    /// Initializes a new `InternalTokenBuilder`.
    pub fn new() -> InternalTokenBuilder<'data> {
        InternalTokenBuilder::default()
    }

    /// Initializess a new `TraitImpl` to help implement the provided trait.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait to implement.
    pub fn implements(trait_ref: TraitVariantRef<'data>) -> TraitImpl<'data> {
        TraitImpl::new(trait_ref)
    }

    /// Returns whether the token stream is public.
    pub fn is_public(&self) -> bool {
        self.publicness.is_public()
    }

    /// Returns whether it implements the given trait.
    pub fn implements_trait(&self, trait_ref: &TraitVariantRef<'data>) -> bool {
        self.implemented_traits.contains(trait_ref)
    }
}

impl ToTokens for InternalToken<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.stream.to_tokens(tokens);
    }
}

impl<'data> InternalDependencies<'data> for InternalToken<'data> {
    fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        let mut dependencies = Vec::new();
        for data in &self.data {
            dependencies.extend(data.internal_dependencies());
        }
        for module in &self.internal_modules {
            dependencies.push(module.internal_crate());
        }
        for trait_ref in &self.employed_traits {
            dependencies.extend(trait_ref.internal_dependencies());
        }
        for trait_ref in &self.implemented_traits {
            dependencies.extend(trait_ref.internal_dependencies());
        }

        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}

impl<'data> ExternalDependencies<'data> for InternalToken<'data> {
    fn external_dependencies(&self) -> Vec<&ExternalCrate<'data>> {
        let mut dependencies = Vec::new();
        for ext_macro in &self.external_macros {
            dependencies.push(ext_macro.external_crate());
        }
        for trait_ref in &self.employed_traits {
            dependencies.extend(trait_ref.external_dependencies());
        }
        for trait_ref in &self.implemented_traits {
            dependencies.extend(trait_ref.external_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}
