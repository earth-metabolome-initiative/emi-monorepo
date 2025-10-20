//! Submodule providing the `Derive` struct which allows to represent derives
//! applied to SynQL internal data.

mod builder;

pub use builder::DeriveBuilder;
use quote::{ToTokens, quote};

use crate::structs::{FeatureFlag, external_trait::TraitVariantRef};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a derive applied to SynQL internal data.
pub struct Derive<'data> {
    /// Features required by the derive.
    features: Vec<FeatureFlag>,
    /// Traits implemented by the derive.
    traits: Vec<TraitVariantRef<'data>>,
}

impl<'data> Derive<'data> {
    /// Initializes a new `DeriveBuilder`.
    pub fn new() -> DeriveBuilder<'data> {
        DeriveBuilder::default()
    }
}

impl ToTokens for Derive<'_> {
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
