//! Submodule providing the `FeatureFlag` struct which allows to represent
//! feature flags used in an crate.

use quote::ToTokens;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a feature flag used in a crate.
pub struct FeatureFlag {
    /// The name of the feature flag.
    name: String,
}

impl FeatureFlag {
    /// Returns the name of the feature flag.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl ToTokens for FeatureFlag {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        tokens.extend(quote::quote! { feature=#name });
    }
}
