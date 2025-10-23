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

    /// Returns the external crates required by the derive.
    pub fn external_dependencies(&self) -> Vec<&'data crate::structs::ExternalCrate<'data>> {
        let mut crates = self
            .traits
            .iter()
            .filter_map(|t| {
                if let TraitVariantRef::External(ext_trait_ref) = t {
                    Some(ext_trait_ref.external_crate())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        crates.sort_unstable();
        crates.dedup();
        crates
    }

    /// Returns the internal crates required by the derive.
    pub fn internal_dependencies(&self) -> Vec<&crate::structs::InternalCrate<'data>> {
        let mut crates = self
            .traits
            .iter()
            .filter_map(|t| {
                if let TraitVariantRef::Internal(_, krate) = t { Some(*krate) } else { None }
            })
            .collect::<Vec<_>>();
        crates.sort_unstable();
        crates.dedup();
        crates
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
