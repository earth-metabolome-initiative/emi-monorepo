//! Submodule providing the `Argument` struct which represents a rust method
//! argument.

mod builder;

pub use builder::ArgumentBuilder;
use quote::ToTokens;

use crate::{
    structs::{Documentation, ExternalCrate, InternalCrate, internal_data::DataVariantRef},
    traits::{ExternalDependencies, InternalDependencies},
    utils::RESERVED_RUST_WORDS,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Struct representing a rust method argument.
pub struct Argument {
    /// Name of the argument.
    name: String,
    /// Type of the argument.
    arg_type: DataVariantRef,
    /// Whether the argument is mutable.
    mutable: bool,
    /// Documentation of the argument.
    documentation: Option<Documentation>,
}

impl Argument {
    /// Returns the name of the argument.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the documentation of the argument.
    #[must_use]
    pub fn documentation(&self) -> Option<&Documentation> {
        self.documentation.as_ref()
    }

    /// Returns the type of the argument.
    #[must_use]
    pub fn arg_type(&self) -> &DataVariantRef {
        &self.arg_type
    }

    /// Returns whether the argument is a self argument.
    #[must_use]
    pub fn is_self(&self) -> bool {
        self.name == "self" && self.arg_type().is_self_type()
    }

    /// Returns whether the argument is a mutable self argument.
    #[must_use]
    pub fn is_mut_self(&self) -> bool {
        self.is_self() && self.arg_type().is_mutable_reference()
    }

    /// Makes the argument mutable.
    pub fn make_mutable(&mut self) {
        self.mutable = true;
    }

    /// Returns the ident of the argument.
    #[must_use]
    pub fn ident(&self) -> syn::Ident {
        if RESERVED_RUST_WORDS.contains(&self.name()) {
            syn::Ident::new_raw(&self.name, proc_macro2::Span::call_site())
        } else {
            syn::Ident::new(&self.name, proc_macro2::Span::call_site())
        }
    }

    /// Returns whether the argument is compatible with another argument.
    #[must_use]
    pub fn is_compatible_with(&self, other: &Argument) -> bool {
        self.name == other.name && self.arg_type == other.arg_type
    }
}

impl Argument {
    /// Initializes a new `ArgumentBuilder`.
    #[must_use]
    pub fn new() -> ArgumentBuilder {
        ArgumentBuilder::default()
    }
}

impl ToTokens for Argument {
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
        let arg_type_tokens = self.arg_type.format_with_generics();
        if self.mutable {
            tokens.extend(quote::quote! { mut #name_ident: #arg_type_tokens });
        } else {
            tokens.extend(quote::quote! { #name_ident: #arg_type_tokens });
        }
    }
}

impl InternalDependencies for Argument {
    #[inline]
    fn internal_dependencies(&self) -> impl Iterator<Item = &InternalCrate> {
        self.arg_type.internal_dependencies().chain(
            self.documentation
                .as_ref()
                .into_iter()
                .flat_map(crate::structs::documentation::Documentation::internal_dependencies),
        )
    }
}

impl ExternalDependencies for Argument {
    #[inline]
    fn external_dependencies(&self) -> impl Iterator<Item = &ExternalCrate> {
        self.arg_type.external_dependencies().chain(
            self.documentation
                .as_ref()
                .into_iter()
                .flat_map(crate::traits::ExternalDependencies::external_dependencies),
        )
    }
}
