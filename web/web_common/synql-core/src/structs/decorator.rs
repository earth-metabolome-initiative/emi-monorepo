//! Submodule providing the `Decorator` struct which allows to represent
//! decorators applied to SynQL internal data.

mod builder;

use std::sync::Arc;

pub use builder::DecoratorBuilder;
use quote::{ToTokens, quote};

use crate::{
    structs::{ExternalCrate, FeatureFlag, InternalToken},
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A decorator applied to SynQL internal data.
pub struct Decorator {
    /// Features required by the derive.
    features: Vec<FeatureFlag>,
    /// Internal token which represents the decorator.
    token: InternalToken,
}

impl Decorator {
    /// Initializes a new `DecoratorBuilder`.
    pub fn new() -> DecoratorBuilder {
        DecoratorBuilder::default()
    }
}

impl ExternalDependencies for Decorator {
    fn external_dependencies(&self) -> Vec<Arc<ExternalCrate>> {
        self.token.external_dependencies()
    }
}

impl InternalDependencies for Decorator {
    fn internal_dependencies(&self) -> Vec<&super::InternalCrate> {
        self.token.internal_dependencies()
    }
}

impl ToTokens for Decorator {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let token = &self.token;
        if !self.features.is_empty() {
            let features = &self.features;
            tokens.extend(quote! {
                #[cfg_attr(all( #(#features),* ), #token)]
            });
        } else {
            tokens.extend(quote! {
                #[#token]
            });
        }
    }
}
