//! Submodule providing a struct which defines a struct model.

use quote::ToTokens;
use syn::Ident;

use crate::structs::{InternalCrate, Publicness, internal_data::DataVariantRef};

#[derive(Debug, Clone)]
/// Struct defining a struct model.
pub struct InternalStruct<'data> {
    /// Attributes of the struct.
    attributes: Vec<(Publicness, Ident, DataVariantRef<'data>)>,
}

impl<'data> InternalStruct<'data> {
    /// Returns a reference to the attributes of the struct.
    pub fn attributes(&self) -> &Vec<(Publicness, Ident, DataVariantRef<'data>)> {
        &self.attributes
    }

    /// Returns the sorted unique internal crate dependencies of the variant.
    pub fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        let mut deps: Vec<&InternalCrate<'data>> =
            self.attributes.iter().flat_map(|(_, _, ty)| ty.internal_dependencies()).collect();
        deps.sort_unstable();
        deps.dedup();
        deps
    }

    /// Returns the sorted unique external crate dependencies of the variant.
    pub fn external_dependencies(&self) -> Vec<&'data crate::structs::ExternalCrate> {
        let mut deps: Vec<&'data crate::structs::ExternalCrate> =
            self.attributes.iter().flat_map(|(_, _, ty)| ty.external_dependencies()).collect();
        deps.sort_unstable();
        deps.dedup();
        deps
    }
}

impl<'data> ToTokens for InternalStruct<'data> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let attrs = self.attributes.iter().map(|(pubness, ident, ty)| {
            let pubness = pubness.to_token_stream();
            quote::quote! { #pubness #ident: #ty }
        });
        let token = quote::quote! {
            {
                #(#attrs),*
            }
        };
        tokens.extend(token);
    }
}
