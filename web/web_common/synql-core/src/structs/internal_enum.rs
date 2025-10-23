//! Submodule providing a struct which defines a enum model.

use quote::ToTokens;
use syn::Ident;

use crate::{
    structs::{external_trait::TraitVariantRef, internal_data::DataVariantRef},
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct defining a enum model.
pub struct InternalEnum<'data> {
    /// Variants of the enum.
    variants: Vec<(Ident, DataVariantRef<'data>)>,
}

impl<'data> InternalDependencies<'data> for InternalEnum<'data> {
    fn internal_dependencies(&self) -> Vec<&crate::structs::InternalCrate<'data>> {
        let mut dependencies = Vec::new();
        for (_, ty) in &self.variants {
            dependencies.extend(ty.internal_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}

impl<'data> ExternalDependencies<'data> for InternalEnum<'data> {
    fn external_dependencies(&self) -> Vec<&crate::structs::ExternalCrate<'data>> {
        let mut dependencies = Vec::new();
        for (_, ty) in &self.variants {
            dependencies.extend(ty.external_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}

impl<'data> InternalEnum<'data> {
    /// Returns a reference to the variants of the enum.
    pub fn variants(&self) -> &Vec<(Ident, DataVariantRef<'data>)> {
        &self.variants
    }

    /// Returns whether the enum supports the given trait.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait variant to check support for.
    pub fn supports_trait(&self, trait_ref: &TraitVariantRef<'data>) -> bool {
        self.variants.iter().all(|(_, ty)| ty.supports_trait(trait_ref))
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
