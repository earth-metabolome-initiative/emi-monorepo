//! Submodule providing a struct which defines a struct model.

mod internal_attribute_builder;
mod internal_struct_builder;

pub use internal_attribute_builder::InternalAttributeBuilder;
pub use internal_struct_builder::InternalStructBuilder;
use quote::ToTokens;
use syn::Ident;

use crate::{
    structs::{
        InternalCrate, Publicness, external_trait::TraitVariantRef, internal_data::DataVariantRef,
    },
    utils::RESERVED_RUST_WORDS,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct defining an attribute of a struct model.
pub struct InternalAttribute<'data> {
    /// Publicness of the attribute.
    pubness: Publicness,
    /// The documentation of the attribute.
    documentation: Option<String>,
    /// Identifier of the attribute.
    name: String,
    /// Type of the attribute.
    ty: DataVariantRef<'data>,
    /// Whether the attribute is nullable.
    nullable: bool,
}

impl<'data> InternalAttribute<'data> {
    /// Initializes a new `InternalAttributeBuilder`.
    pub fn new() -> InternalAttributeBuilder<'data> {
        InternalAttributeBuilder::default()
    }

    /// Returns the publicness of the attribute.
    pub fn pubness(&self) -> &Publicness {
        &self.pubness
    }

    /// Returns the name of the attribute.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the ident of the attribute.
    pub fn ident(&self) -> Ident {
        if RESERVED_RUST_WORDS.contains(&self.name()) {
            Ident::new_raw(&self.name, proc_macro2::Span::call_site())
        } else {
            Ident::new(&self.name, proc_macro2::Span::call_site())
        }
    }

    /// Returns the documentation of the attribute.
    pub fn documentation(&self) -> Option<&str> {
        self.documentation.as_deref()
    }

    /// Returns the type of the attribute.
    pub fn ty(&self) -> &DataVariantRef<'data> {
        &self.ty
    }

    /// Returns whether the attribute is nullable.
    pub fn is_nullable(&self) -> bool {
        self.nullable
    }

    /// Returns the internal crate dependencies of the attribute's type.
    pub fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        match &self.ty {
            DataVariantRef::Internal(internal) => internal.internal_dependencies(),
            DataVariantRef::External(_) => vec![],
        }
    }

    /// Returns the external crate dependencies of the attribute's type.
    pub fn external_dependencies(&self) -> Vec<&crate::structs::ExternalCrate<'data>> {
        match &self.ty {
            DataVariantRef::Internal(_) => vec![],
            DataVariantRef::External(external) => vec![external.external_crate()],
        }
    }

    /// Returns whether the attribute's type supports the given trait.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait variant to check support for.
    pub fn supports_trait(&self, trait_ref: &TraitVariantRef<'data>) -> bool {
        self.ty.supports_trait(trait_ref)
    }
}

impl ToTokens for InternalAttribute<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let pubness = self.pubness.to_token_stream();
        let ident = self.ident();
        let documentation = match &self.documentation {
            Some(doc) => {
                quote::quote! {
                    #[doc = #doc]
                }
            }
            None => quote::quote! {},
        };
        let ty = &self.ty;

        let formatted_ty = if self.is_nullable() {
            quote::quote! {Option<#ty>}
        } else {
            quote::quote! {#ty}
        };

        let token = quote::quote! {
            #documentation
            #pubness #ident: #formatted_ty
        };
        tokens.extend(token);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct defining a struct model.
pub struct InternalStruct<'data> {
    /// Attributes of the struct.
    attributes: Vec<InternalAttribute<'data>>,
}

impl<'data> InternalStruct<'data> {
    /// Initializes a new `InternalStructBuilder`.
    pub fn new() -> InternalStructBuilder<'data> {
        InternalStructBuilder::default()
    }

    /// Returns a reference to the attributes of the struct.
    pub fn attributes(&self) -> &Vec<InternalAttribute<'data>> {
        &self.attributes
    }

    /// Returns the sorted unique internal crate dependencies of the variant.
    pub fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        let mut deps: Vec<&InternalCrate<'data>> =
            self.attributes.iter().flat_map(|attr| attr.internal_dependencies()).collect();
        deps.sort_unstable();
        deps.dedup();
        deps
    }

    /// Returns the sorted unique external crate dependencies of the variant.
    pub fn external_dependencies(&self) -> Vec<&crate::structs::ExternalCrate<'data>> {
        let mut deps: Vec<&crate::structs::ExternalCrate<'data>> =
            self.attributes.iter().flat_map(|attr| attr.external_dependencies()).collect();
        deps.sort_unstable();
        deps.dedup();
        deps
    }

    /// Returns whether the struct supports the given trait.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait variant to check support for.
    pub fn supports_trait(&self, trait_ref: &TraitVariantRef<'data>) -> bool {
        self.attributes.iter().all(|attr| attr.supports_trait(trait_ref))
    }
}

impl<'data> ToTokens for InternalStruct<'data> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let attrs = &self.attributes;
        let token = quote::quote! {
            {
                #(#attrs),*
            }
        };
        tokens.extend(token);
    }
}
