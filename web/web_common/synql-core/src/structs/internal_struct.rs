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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct defining an attribute of a struct model.
pub struct InternalAttribute<'data> {
    /// Publicness of the attribute.
    pubness: Publicness,
    /// The documentation of the attribute.
    documentation: Documentation<'data>,
    /// Identifier of the attribute.
    name: String,
    /// Type of the attribute.
    ty: DataVariantRef<'data>,
}

impl<'data> InternalAttribute<'data> {
    /// Initializes a new `InternalAttributeBuilder`.
    pub fn new() -> InternalAttributeBuilder<'data> {
        InternalAttributeBuilder::default()
    }

    /// Returns a variant of the current attribute with the type
    /// made optional.
    pub fn optional(&self) -> InternalAttribute<'data> {
        let mut new_attr = self.clone();
        if !new_attr.ty.is_option() {
            new_attr.ty = new_attr.ty.optional();
        }
        new_attr
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
    pub fn documentation(&self) -> &Documentation<'data> {
        &self.documentation
    }

    /// Returns the type of the attribute.
    pub fn ty(&self) -> &DataVariantRef<'data> {
        &self.ty
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

impl<'data> InternalDependencies<'data> for InternalAttribute<'data> {
    fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        let mut dependencies = self.ty.internal_dependencies();
        dependencies.extend(self.documentation.internal_dependencies());
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}

impl<'data> ExternalDependencies<'data> for InternalAttribute<'data> {
    fn external_dependencies(&self) -> Vec<&crate::structs::ExternalCrate<'data>> {
        let mut dependencies = self.ty.external_dependencies();
        dependencies.extend(self.documentation.external_dependencies());
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}

impl ToTokens for InternalAttribute<'_> {
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

impl<'data> InternalDependencies<'data> for InternalStruct<'data> {
    fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        let mut deps: Vec<&InternalCrate<'data>> =
            self.attributes.iter().flat_map(|attr| attr.internal_dependencies()).collect();
        deps.sort_unstable();
        deps.dedup();
        deps
    }
}

impl<'data> ExternalDependencies<'data> for InternalStruct<'data> {
    fn external_dependencies(&self) -> Vec<&crate::structs::ExternalCrate<'data>> {
        let mut deps: Vec<&crate::structs::ExternalCrate<'data>> =
            self.attributes.iter().flat_map(|attr| attr.external_dependencies()).collect();
        deps.sort_unstable();
        deps.dedup();
        deps
    }
}
