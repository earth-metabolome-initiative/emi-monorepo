//! Submodule providing a struct which defines a enum model.

use quote::ToTokens;
use syn::Ident;

use crate::structs::internal_data::DataVariantRef;

#[derive(Debug, Clone)]
/// Struct defining a enum model.
pub struct InternalEnum<'data> {
    /// Variants of the enum.
    variants: Vec<(Ident, DataVariantRef<'data>)>,
}

impl<'data> InternalEnum<'data> {
    /// Returns a reference to the variants of the enum.
    pub fn variants(&self) -> &Vec<(Ident, DataVariantRef<'data>)> {
        &self.variants
    }

    /// Returns the internal crate dependencies of the enum.
    pub fn internal_dependencies(&self) -> Vec<&'data crate::structs::InternalCrate<'data>> {
        let mut dependencies = Vec::new();
        for (_, ty) in &self.variants {
            dependencies.extend(ty.internal_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }

    /// Returns the external crate dependencies of the enum.
    pub fn external_dependencies(&self) -> Vec<&'data crate::structs::ExternalCrate> {
        let mut dependencies = Vec::new();
        for (_, ty) in &self.variants {
            dependencies.extend(ty.external_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}

impl<'data> ToTokens for InternalEnum<'data> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let variants = self.variants.iter().map(|(ident, ty)| {
            quote::quote! { #ident(#ty) }
        });
        let token = quote::quote! {
            {
                #(#variants),*
            }
        };
        tokens.extend(token);
    }
}
