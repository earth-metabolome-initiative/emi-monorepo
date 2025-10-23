//! Submodule providing the `Decorator` struct which allows to represent
//! decorators applied to SynQL internal data.

mod builder;

pub use builder::DecoratorBuilder;
use quote::{ToTokens, quote};

use crate::{
    structs::{FeatureFlag, InternalToken},
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A decorator applied to SynQL internal data.
pub struct Decorator<'data> {
    /// Features required by the derive.
    features: Vec<FeatureFlag>,
    /// Internal token which represents the decorator.
    token: InternalToken<'data>,
}

impl<'data> Decorator<'data> {
    /// Initializes a new `DecoratorBuilder`.
    pub fn new() -> DecoratorBuilder<'data> {
        DecoratorBuilder::default()
    }
}

impl<'data> ExternalDependencies<'data> for Decorator<'data> {
    fn external_dependencies(&self) -> Vec<&super::ExternalCrate<'data>> {
        self.token.external_dependencies()
    }
}

impl<'data> InternalDependencies<'data> for Decorator<'data> {
    fn internal_dependencies(&self) -> Vec<&super::InternalCrate<'data>> {
        self.token.internal_dependencies()
    }
}

impl ToTokens for Decorator<'_> {
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
