//! Submodule providing a struct which defines a enum model.

mod internal_enum_builder;
mod internal_variant_builder;

use std::sync::Arc;

pub use internal_enum_builder::InternalEnumBuilder;
pub use internal_variant_builder::InternalVariantBuilder;
use quote::ToTokens;
use syn::Ident;

use crate::{
    structs::{
        Documentation, InternalCrate, external_trait::TraitVariantRef,
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
    fn internal_dependencies(&self) -> Vec<&InternalCrate> {
        let mut dependencies = self.ty.internal_dependencies();
        dependencies.extend(self.doc.internal_dependencies());
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}

impl ExternalDependencies for InternalVariant {
    fn external_dependencies(&self) -> Vec<Arc<crate::structs::ExternalCrate>> {
        let mut dependencies = self.ty.external_dependencies();
        dependencies.extend(self.doc.external_dependencies());
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
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
    fn internal_dependencies(&self) -> Vec<&crate::structs::InternalCrate> {
        let mut dependencies = Vec::new();
        for variant in &self.variants {
            dependencies.extend(variant.internal_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}

impl ExternalDependencies for InternalEnum {
    fn external_dependencies(&self) -> Vec<Arc<crate::structs::ExternalCrate>> {
        let mut dependencies = Vec::new();
        for variant in &self.variants {
            dependencies.extend(variant.external_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
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
