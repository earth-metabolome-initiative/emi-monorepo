//! Submodule providing the `Argument` struct which represents a rust method
//! argument.

mod builder;
pub use builder::{ArgumentAttribute, ArgumentBuilder, ArgumentBuilderError};
use quote::ToTokens;

use crate::{
    structs::{Documentation, internal_data::DataVariantRef},
    traits::{ExternalDependencies, InternalDependencies},
    utils::RESERVED_RUST_WORDS,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a rust method argument.
pub struct Argument<'data> {
    /// Name of the argument.
    name: String,
    /// Type of the argument.
    arg_type: DataVariantRef<'data>,
    /// Whether the argument is mutable.
    mutable: bool,
    /// Documentation of the argument.
    documentation: Option<Documentation<'data>>,
}

impl<'data> Argument<'data> {
    /// Returns the name of the argument.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the documentation of the argument.
    pub fn documentation(&self) -> Option<&Documentation<'data>> {
        self.documentation.as_ref()
    }

    /// Returns the type of the argument.
    pub fn arg_type(&self) -> &DataVariantRef<'data> {
        &self.arg_type
    }

    /// Returns whether the argument is a self argument.
    pub fn is_self(&self) -> bool {
        self.name == "self" && self.arg_type().is_self_type()
    }

    /// Makes the argument mutable.
    pub fn make_mutable(&mut self) {
        self.mutable = true;
    }

    /// Returns the ident of the argument.
    pub fn ident(&self) -> syn::Ident {
        if RESERVED_RUST_WORDS.contains(&self.name()) {
            syn::Ident::new_raw(&self.name, proc_macro2::Span::call_site())
        } else {
            syn::Ident::new(&self.name, proc_macro2::Span::call_site())
        }
    }

    /// Returns whether the argument is compatible with another argument.
    pub fn is_compatible_with(&self, other: &Argument<'_>) -> bool {
        self.name == other.name && self.arg_type == other.arg_type
    }
}

impl<'data> Argument<'data> {
    /// Initializes a new `ArgumentBuilder`.
    pub fn new() -> ArgumentBuilder<'data> {
        ArgumentBuilder::default()
    }
}

impl ToTokens for Argument<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if self.is_self() {
            if self.arg_type.is_reference() {
                tokens.extend(quote::quote! { &self });
            } else if self.arg_type.is_mutable_reference() {
                tokens.extend(quote::quote! { &mut self });
            } else if self.mutable {
                tokens.extend(quote::quote! { mut self });
            } else {
                tokens.extend(quote::quote! { self });
            }
            return;
        }
        let name_ident = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        let arg_type_tokens = self.arg_type.to_token_stream();
        if self.mutable {
            tokens.extend(quote::quote! { mut #name_ident: #arg_type_tokens });
        } else {
            tokens.extend(quote::quote! { #name_ident: #arg_type_tokens });
        }
    }
}

impl<'data> InternalDependencies<'data> for Argument<'data> {
    fn internal_dependencies(&self) -> Vec<&crate::structs::InternalCrate<'data>> {
        let mut dependencies = self.arg_type.internal_dependencies();
        if let Some(doc) = &self.documentation {
            dependencies.extend(doc.internal_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}

impl<'data> ExternalDependencies<'data> for Argument<'data> {
    fn external_dependencies(&self) -> Vec<&crate::structs::ExternalCrate<'data>> {
        let mut dependencies = self.arg_type.external_dependencies();
        if let Some(doc) = &self.documentation {
            dependencies.extend(doc.external_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}
