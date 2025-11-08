//! Submodule providing the `Derive` struct which allows to represent derives
//! applied to SynQL internal data.

mod builder;

use std::sync::Arc;

pub use builder::DeriveBuilder;
use quote::{ToTokens, quote};

use crate::{
    structs::{ExternalCrate, FeatureFlag, external_trait::TraitVariantRef},
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a derive applied to SynQL internal data.
pub struct Derive {
    /// Features required by the derive.
    features: Vec<FeatureFlag>,
    /// Traits implemented by the derive.
    traits: Vec<TraitVariantRef>,
}

impl Derive {
    /// Initializes a new `DeriveBuilder`.
    pub fn new() -> DeriveBuilder {
        DeriveBuilder::default()
    }
}

impl ExternalDependencies for Derive {
    fn external_dependencies(&self) -> Vec<Arc<ExternalCrate>> {
        let mut crates = self
            .traits
            .iter()
            .filter_map(|t| {
                if let TraitVariantRef::External(ext_trait_ref) = t {
                    Some(Arc::new(ext_trait_ref.external_crate().clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        crates.sort_unstable();
        crates.dedup();
        crates
    }
}

impl InternalDependencies for Derive {
    fn internal_dependencies(&self) -> Vec<&super::InternalCrate> {
        let mut crates = self
            .traits
            .iter()
            .filter_map(|t| {
                if let TraitVariantRef::Internal(_, krate) = t { krate.as_deref() } else { None }
            })
            .collect::<Vec<_>>();
        crates.sort_unstable();
        crates.dedup();
        crates
    }
}

impl ToTokens for Derive {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let derive_statement = {
            let traits = &self.traits;
            quote! {
                derive( #(#traits),* )
            }
        };
        if !self.features.is_empty() {
            let features = &self.features;
            tokens.extend(quote! {
                #[cfg_attr(all( #(#features),* ), #derive_statement)]
            });
        } else {
            tokens.extend(quote! {
                #[#derive_statement]
            });
        }
    }
}
