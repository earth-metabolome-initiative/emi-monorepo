//! Submodule providing a struct which defines a enum model.

mod internal_enum_builder;
mod internal_variant_builder;

pub use internal_enum_builder::InternalEnumBuilder;
pub use internal_variant_builder::InternalVariantBuilder;
use quote::ToTokens;
use syn::Ident;

use crate::{
    structs::{
        Documentation, ExternalCrate, InternalCrate, external_trait::TraitVariantRef,
        internal_data::DataVariantRef,
    },
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Struct defining a variant of an enum model.
pub struct InternalVariant {
    /// Name of the variant.
    name: Ident,
    /// Documentation comment of the variant.
    doc: Documentation,
    /// Type of the variant.
    ty: Option<DataVariantRef>,
}

impl InternalVariant {
    /// Initializes a new `InternalVariantBuilder`.
    pub fn new() -> InternalVariantBuilder {
        InternalVariantBuilder::default()
    }

    /// Returns the ident of the variant.
    pub fn ident(&self) -> &Ident {
        &self.name
    }

    /// Returns the documentation comment of the variant.
    pub fn doc(&self) -> &Documentation {
        &self.doc
    }

    /// Returns the type of the variant.
    pub fn ty(&self) -> Option<&DataVariantRef> {
        self.ty.as_ref()
    }

    /// Returns whether the variant's type supports the given trait.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait variant to check support for.
    pub fn supports_trait(&self, trait_ref: &TraitVariantRef) -> bool {
        if let Some(ty) = &self.ty {
            ty.supports_trait(trait_ref)
        } else {
            trait_ref.implemented_for_typeless_enum()
        }
    }
}

impl InternalDependencies for InternalVariant {
    #[inline]
    fn internal_dependencies(&self) -> impl Iterator<Item = &InternalCrate> {
        self.ty.internal_dependencies().chain(self.doc.internal_dependencies())
    }
}

impl ExternalDependencies for InternalVariant {
    #[inline]
    fn external_dependencies(&self) -> impl Iterator<Item = &ExternalCrate> {
        self.ty.external_dependencies().chain(self.doc.external_dependencies())
    }
}

impl ToTokens for InternalVariant {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let doc = &self.doc;
        tokens.extend(quote::quote! {
            #doc
        });

        if let Some(ty) = &self.ty {
            tokens.extend(quote::quote! {
                #name(#ty)
            });
        } else {
            tokens.extend(quote::quote! {
                #name
            });
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Struct defining a enum model.
pub struct InternalEnum {
    /// Variants of the enum.
    variants: Vec<InternalVariant>,
}

impl InternalDependencies for InternalEnum {
    #[inline]
    fn internal_dependencies(&self) -> impl Iterator<Item = &crate::structs::InternalCrate> {
        self.variants.iter().flat_map(|variant| variant.internal_dependencies())
    }
}

impl ExternalDependencies for InternalEnum {
    #[inline]
    fn external_dependencies(&self) -> impl Iterator<Item = &ExternalCrate> {
        self.variants.iter().flat_map(|variant| variant.external_dependencies())
    }
}

impl InternalEnum {
    /// Initializes a new `InternalEnumBuilder`.
    pub fn new() -> InternalEnumBuilder {
        InternalEnumBuilder::default()
    }

    /// Returns a reference to the variants of the enum.
    pub fn variants(&self) -> &Vec<InternalVariant> {
        &self.variants
    }

    /// Returns whether the enum supports the given trait.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait variant to check support for.
    pub fn supports_trait(&self, trait_ref: &TraitVariantRef) -> bool {
        self.variants.iter().all(|variant| variant.supports_trait(trait_ref))
    }
}

impl ToTokens for InternalEnum {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let variants = &self.variants;
        let token = quote::quote! {
            {
                #(#variants),*
            }
        };
        tokens.extend(token);
    }
}
