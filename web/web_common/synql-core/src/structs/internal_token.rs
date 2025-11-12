//! Submodule defining a struct which represents a tokenstream alongside its
//! metadata.

mod builder;

use std::hash::Hash;

pub use builder::InternalTokenBuilder;
use common_traits::prelude::Builder;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

mod from_ident;
mod trait_impl;

pub use trait_impl::TraitImpl;

use crate::{
    structs::{
        DataVariantRef, ExternalCrate, ExternalFunctionRef, InternalCrate, Publicness,
        external_crate::ExternalMacroRef, external_trait::TraitVariantRef,
        internal_data::InternalModuleRef,
    },
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Debug, Clone)]
/// Struct representing a rust tokenstream alongside its metadata.
pub struct InternalToken {
    /// Publicness of the token stream.
    publicness: Publicness,
    /// The token stream.
    stream: TokenStream,
    /// External macros used in the token stream.
    external_macros: Vec<ExternalMacroRef>,
    /// Traits used in the token stream.
    employed_traits: Vec<TraitVariantRef>,
    /// Employed functions.
    employed_functions: Vec<ExternalFunctionRef>,
    /// Traits which are implemented by the token stream.
    implemented_traits: Vec<TraitVariantRef>,
    /// Data used in the token stream.
    data: Vec<DataVariantRef>,
    /// Internal modules from other crates in the same workspace which are used
    /// in the token stream.
    internal_modules: Vec<InternalModuleRef>,
}

impl From<Vec<InternalToken>> for InternalToken {
    fn from(tokens: Vec<InternalToken>) -> Self {
        InternalToken::new()
            .private()
            .inherits(tokens.iter().cloned())
            .stream(quote! {#( #tokens )*})
            .build()
            .unwrap()
    }
}

impl PartialEq for InternalToken {
    fn eq(&self, other: &Self) -> bool {
        self.publicness == other.publicness
            && self.external_macros == other.external_macros
            && self.employed_traits == other.employed_traits
            && self.implemented_traits == other.implemented_traits
            && self.data == other.data
            && self.internal_modules == other.internal_modules
            && self.stream.to_string() == other.stream.to_string()
    }
}

impl Eq for InternalToken {}

impl Hash for InternalToken {
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

impl InternalToken {
    /// Initializes a new `InternalTokenBuilder`.
    pub fn new() -> InternalTokenBuilder {
        InternalTokenBuilder::default()
    }

    /// Initializes a new `TraitImpl` to help implement the provided trait.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait to implement.
    pub fn implements<'trt>(trait_ref: &'trt TraitVariantRef) -> TraitImpl<'trt> {
        TraitImpl::new(trait_ref)
    }

    /// Returns whether the token stream is public.
    #[inline]
    pub fn is_public(&self) -> bool {
        self.publicness.is_public()
    }

    /// Returns whether it implements the given trait.
    #[inline]
    pub fn implements_trait(&self, trait_ref: &TraitVariantRef) -> bool {
        self.implemented_traits.contains(trait_ref)
    }
}

impl From<TokenStream> for InternalToken {
    fn from(stream: TokenStream) -> Self {
        InternalToken {
            publicness: Publicness::Private,
            stream,
            external_macros: Vec::new(),
            employed_traits: Vec::new(),
            implemented_traits: Vec::new(),
            employed_functions: Vec::new(),
            data: Vec::new(),
            internal_modules: Vec::new(),
        }
    }
}

impl From<syn::GenericParam> for InternalToken {
    fn from(generic: syn::GenericParam) -> Self {
        InternalToken::new().private().stream(quote! { #generic }).build().unwrap()
    }
}

impl From<DataVariantRef> for InternalToken {
    fn from(data: DataVariantRef) -> Self {
        InternalToken::new().private().stream(quote! { #data }).data(data).build().unwrap()
    }
}

impl ToTokens for InternalToken {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.stream.to_tokens(tokens);
    }
}

impl InternalDependencies for InternalToken {
    #[inline]
    fn internal_dependencies(&self) -> impl Iterator<Item = &InternalCrate> {
        self.data
            .iter()
            .flat_map(|data| data.internal_dependencies())
            .chain(self.internal_modules.iter().flat_map(|module| module.internal_dependencies()))
            .chain(
                self.employed_traits.iter().flat_map(|trait_ref| trait_ref.internal_dependencies()),
            )
            .chain(
                self.implemented_traits
                    .iter()
                    .flat_map(|trait_ref| trait_ref.internal_dependencies()),
            )
    }
}

impl ExternalDependencies for InternalToken {
    #[inline]
    fn external_dependencies(&self) -> impl Iterator<Item = &ExternalCrate> {
        self.external_macros
            .iter()
            .flat_map(|ext_macro| ext_macro.external_dependencies())
            .chain(
                self.employed_traits.iter().flat_map(|trait_ref| trait_ref.external_dependencies()),
            )
            .chain(
                self.implemented_traits
                    .iter()
                    .flat_map(|trait_ref| trait_ref.external_dependencies()),
            )
            .chain(self.data.iter().flat_map(|data| data.external_dependencies()))
            .chain(self.employed_functions.iter().flat_map(|func| func.external_dependencies()))
    }
}
