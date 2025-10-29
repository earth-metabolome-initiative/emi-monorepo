//! Submodule providing a struct which defines a enum model.

mod internal_enum_builder;
mod internal_variant_builder;

pub use internal_enum_builder::InternalEnumBuilder;
pub use internal_variant_builder::InternalVariantBuilder;
use quote::ToTokens;
use syn::Ident;

use crate::{
    structs::{InternalCrate, external_trait::TraitVariantRef, internal_data::DataVariantRef},
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct defining a variant of an enum model.
pub struct InternalVariant<'data> {
    /// Name of the variant.
    name: Ident,
    /// Documentation comment of the variant.
    doc: String,
    /// Type of the variant.
    ty: Option<DataVariantRef<'data>>,
}

impl<'data> InternalVariant<'data> {
    /// Initializes a new `InternalVariantBuilder`.
    pub fn new() -> InternalVariantBuilder<'data> {
        InternalVariantBuilder::default()
    }

    /// Returns the ident of the variant.
    pub fn ident(&self) -> &Ident {
        &self.name
    }

    /// Returns the documentation comment of the variant.
    pub fn doc(&self) -> &str {
        &self.doc
    }

    /// Returns the type of the variant.
    pub fn ty(&self) -> Option<&DataVariantRef<'data>> {
        self.ty.as_ref()
    }

    /// Returns whether the variant's type supports the given trait.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait variant to check support for.
    pub fn supports_trait(&self, trait_ref: &TraitVariantRef<'data>) -> bool {
        if let Some(ty) = &self.ty {
            ty.supports_trait(trait_ref)
        } else {
            trait_ref.implemented_for_typeless_enum()
        }
    }
}

impl<'data> InternalDependencies<'data> for InternalVariant<'data> {
    fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        self.ty.internal_dependencies()
    }
}

impl<'data> ExternalDependencies<'data> for InternalVariant<'data> {
    fn external_dependencies(&self) -> Vec<&crate::structs::ExternalCrate<'data>> {
        self.ty.external_dependencies()
    }
}

impl ToTokens for InternalVariant<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let doc = &self.doc;
        let doc_token = quote::quote! {
            #[doc = #doc]
        };
        tokens.extend(doc_token);

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct defining a enum model.
pub struct InternalEnum<'data> {
    /// Variants of the enum.
    variants: Vec<InternalVariant<'data>>,
}

impl<'data> InternalDependencies<'data> for InternalEnum<'data> {
    fn internal_dependencies(&self) -> Vec<&crate::structs::InternalCrate<'data>> {
        let mut dependencies = Vec::new();
        for variant in &self.variants {
            dependencies.extend(variant.internal_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}

impl<'data> ExternalDependencies<'data> for InternalEnum<'data> {
    fn external_dependencies(&self) -> Vec<&crate::structs::ExternalCrate<'data>> {
        let mut dependencies = Vec::new();
        for variant in &self.variants {
            dependencies.extend(variant.external_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}

impl<'data> InternalEnum<'data> {
    /// Initializes a new `InternalEnumBuilder`.
    pub fn new() -> InternalEnumBuilder<'data> {
        InternalEnumBuilder::default()
    }

    /// Returns a reference to the variants of the enum.
    pub fn variants(&self) -> &Vec<InternalVariant<'data>> {
        &self.variants
    }

    /// Returns whether the enum supports the given trait.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait variant to check support for.
    pub fn supports_trait(&self, trait_ref: &TraitVariantRef<'data>) -> bool {
        self.variants.iter().all(|variant| variant.supports_trait(trait_ref))
    }
}

impl<'data> ToTokens for InternalEnum<'data> {
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
