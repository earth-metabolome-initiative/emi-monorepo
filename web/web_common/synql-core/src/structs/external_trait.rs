//! Submodule providing a struct defining a trait available in an external
//! crate.

mod builder;
use std::{fmt::Debug, hash::Hash};

use builder::ExternalTraitBuilder;
use quote::ToTokens;

use crate::structs::{ExternalCrate, InternalCrate, InternalTrait};

#[derive(Clone)]
/// Struct defining a trait available in an external crate.
pub struct ExternalTrait {
    /// The name of the trait.
    name: String,
    /// The [`syn::Path`](syn::Path) representing the trait
    /// within the external crate.
    path: syn::Path,
}

impl Debug for ExternalTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExternalTrait")
            .field("name", &self.name)
            .field("path", &self.path.to_token_stream().to_string())
            .finish()
    }
}

impl PartialEq for ExternalTrait {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.path.to_token_stream().to_string() == other.path.to_token_stream().to_string()
    }
}

impl Eq for ExternalTrait {}

impl Hash for ExternalTrait {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.path.to_token_stream().to_string().hash(state);
    }
}

impl PartialOrd for ExternalTrait {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ExternalTrait {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.name.cmp(&other.name) {
            std::cmp::Ordering::Equal => {
                self.path
                    .to_token_stream()
                    .to_string()
                    .cmp(&other.path.to_token_stream().to_string())
            }
            ord => ord,
        }
    }
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

    /// Returns the [`syn::Path`](syn::Path) representing the trait
    /// within the external crate.
    pub fn path(&self) -> &syn::Path {
        &self.path
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Enum representing a trait implemented for some internal data,
/// which may be defined within the workspace or come from an external crate.
pub enum TraitVariantRef<'data> {
    /// Variant representing a trait defined within the workspace.
    Internal(InternalTrait<'data>, &'data InternalCrate<'data>),
    /// Variant representing a trait defined within an external crate.
    External(ExternalTrait, &'data ExternalCrate),
}

impl ToTokens for TraitVariantRef<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            TraitVariantRef::Internal(trait_def, crate_def) => {
                let trait_ident = trait_def.ident();
                let crate_name = crate_def.name();
                tokens.extend(quote::quote! {
                    #crate_name::prelude::#trait_ident
                });
            }
            TraitVariantRef::External(trait_def, _crate_def) => {
                let trait_path = trait_def.path();
                tokens.extend(quote::quote! {
                    #trait_path
                });
            }
        }
    }
}
