//! Submodule implementing dynamic type casting for syn.

use proc_macro2::TokenStream;

use crate::structs::DataVariantRef;

impl DataVariantRef {
    /// Casts a string value to the appropriate type for this data variant.
    pub fn cast(&self, value: &str) -> Result<TokenStream, syn::Error> {
        if let DataVariantRef::External(external_type) = self {
            return external_type.cast(value);
        }
        Err(syn::Error::new_spanned(
            quote::quote! { #value },
            format!("Cannot cast value to non-external type variant: {:?}", self),
        ))
    }
}
