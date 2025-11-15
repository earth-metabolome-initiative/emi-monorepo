//! Submodule providing the `Derive` struct which allows to represent derives
//! applied to SynQL internal data.

mod builder;

pub use builder::DeriveBuilder;
use quote::{ToTokens, quote};

use crate::{
    structs::{ExternalCrate, FeatureFlag, external_trait::TraitVariantRef},
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Struct representing a derive applied to SynQL internal data.
pub struct Derive {
    /// Features required by the derive.
    features: Vec<FeatureFlag>,
    /// Traits implemented by the derive.
    traits: Vec<TraitVariantRef>,
}

impl Derive {
    /// Initializes a new `DeriveBuilder`.
    #[must_use]
    pub fn new() -> DeriveBuilder {
        DeriveBuilder::default()
    }
}

impl ExternalDependencies for Derive {
    #[inline]
    fn external_dependencies(&self) -> impl Iterator<Item = &ExternalCrate> {
        self.traits.iter().flat_map(ExternalDependencies::external_dependencies)
    }
}

impl InternalDependencies for Derive {
    #[inline]
    fn internal_dependencies(&self) -> impl Iterator<Item = &super::InternalCrate> {
        self.traits.iter().filter_map(|t| {
            if let TraitVariantRef::Internal(_, krate) = t { krate.as_deref() } else { None }
        })
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
        if self.features.is_empty() {
            tokens.extend(quote! {
                #[#derive_statement]
            });
        } else {
            let features = &self.features;
            tokens.extend(quote! {
                #[cfg_attr(all( #(#features),* ), #derive_statement)]
            });
        }
    }
}
