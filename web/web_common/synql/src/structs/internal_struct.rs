//! Submodule providing a struct which defines a struct model.

mod internal_attribute_builder;
mod internal_struct_builder;
mod method;

pub use internal_attribute_builder::InternalAttributeBuilder;
pub use internal_struct_builder::InternalStructBuilder;
pub use method::{Argument, Method, MethodBuilder, WhereClause};
use quote::ToTokens;
use syn::Ident;

use crate::{
    structs::{
        Documentation, InternalCrate, Publicness, external_trait::TraitVariantRef,
        internal_data::DataVariantRef,
    },
    traits::{ExternalDependencies, InternalDependencies},
    utils::RESERVED_RUST_WORDS,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Struct defining an attribute of a struct model.
pub struct InternalAttribute {
    /// Publicness of the attribute.
    pubness: Publicness,
    /// The documentation of the attribute.
    documentation: Documentation,
    /// Identifier of the attribute.
    name: String,
    /// Type of the attribute.
    ty: DataVariantRef,
}

impl InternalAttribute {
    /// Initializes a new `InternalAttributeBuilder`.
    #[must_use]
    pub fn new() -> InternalAttributeBuilder {
        InternalAttributeBuilder::default()
    }

    /// Returns a variant of the current attribute with the type
    /// made optional.
    #[must_use]
    pub fn optional(&self) -> InternalAttribute {
        let mut new_attr = self.clone();
        if !new_attr.ty.is_option() {
            new_attr.ty = new_attr.ty.optional();
        }
        new_attr
    }

    /// Sets the publicness of the attribute to private and returns
    /// the modified attribute.
    #[must_use]
    pub fn private(mut self) -> InternalAttribute {
        self.pubness = Publicness::Private;
        self
    }

    /// Returns the publicness of the attribute.
    #[inline]
    #[must_use]
    pub fn pubness(&self) -> &Publicness {
        &self.pubness
    }

    /// Returns the name of the attribute.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the ident of the attribute.
    #[inline]
    #[must_use]
    pub fn ident(&self) -> Ident {
        if RESERVED_RUST_WORDS.contains(&self.name()) {
            Ident::new_raw(&self.name, proc_macro2::Span::call_site())
        } else {
            Ident::new(&self.name, proc_macro2::Span::call_site())
        }
    }

    /// Returns the documentation of the attribute.
    #[inline]
    #[must_use]
    pub fn documentation(&self) -> &Documentation {
        &self.documentation
    }

    /// Returns the type of the attribute.
    #[inline]
    #[must_use]
    pub fn ty(&self) -> &DataVariantRef {
        &self.ty
    }

    /// Returns whether the attribute's type supports the given trait.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait variant to check support for.
    #[inline]
    #[must_use]
    pub fn supports_trait(&self, trait_ref: &TraitVariantRef) -> bool {
        self.ty.supports_trait(trait_ref)
    }
}

impl InternalDependencies for InternalAttribute {
    #[inline]
    fn internal_dependencies(&self) -> impl Iterator<Item = &InternalCrate> {
        self.ty.internal_dependencies().chain(self.documentation.internal_dependencies())
    }
}

impl ExternalDependencies for InternalAttribute {
    #[inline]
    fn external_dependencies(&self) -> impl Iterator<Item = &crate::structs::ExternalCrate> {
        self.ty.external_dependencies().chain(self.documentation.external_dependencies())
    }
}

impl ToTokens for InternalAttribute {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let pubness = self.pubness.to_token_stream();
        let ident = self.ident();
        let documentation = &self.documentation;
        let ty = &self.ty;

        let token = quote::quote! {
            #documentation
            #pubness #ident: #ty
        };
        tokens.extend(token);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Struct defining a struct model.
pub struct InternalStruct {
    /// Attributes of the struct.
    attributes: Vec<InternalAttribute>,
}

impl InternalStruct {
    /// Initializes a new `InternalStructBuilder`.
    #[must_use]
    pub fn new() -> InternalStructBuilder {
        InternalStructBuilder::default()
    }

    /// Returns a reference to the attributes of the struct.
    #[inline]
    #[must_use]
    pub fn attributes(&self) -> &Vec<InternalAttribute> {
        &self.attributes
    }

    /// Returns whether the struct supports the given trait.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait variant to check support for.
    #[inline]
    #[must_use]
    pub fn supports_trait(&self, trait_ref: &TraitVariantRef) -> bool {
        self.attributes.iter().all(|attr| attr.supports_trait(trait_ref))
    }
}

impl ToTokens for InternalStruct {
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

impl InternalDependencies for InternalStruct {
    #[inline]
    fn internal_dependencies(&self) -> impl Iterator<Item = &InternalCrate> {
        self.attributes.iter().flat_map(InternalAttribute::internal_dependencies)
    }
}

impl ExternalDependencies for InternalStruct {
    #[inline]
    fn external_dependencies(&self) -> impl Iterator<Item = &crate::structs::ExternalCrate> {
        self.attributes.iter().flat_map(crate::traits::ExternalDependencies::external_dependencies)
    }
}
