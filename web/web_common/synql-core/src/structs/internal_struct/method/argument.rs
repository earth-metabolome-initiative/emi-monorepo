//! Submodule providing the `Argument` struct which represents a rust method
//! argument.

mod builder;
use std::sync::Arc;

pub use builder::ArgumentBuilder;
use quote::ToTokens;

use crate::{
    structs::{Documentation, internal_data::DataVariantRef},
    traits::{ExternalDependencies, InternalDependencies},
    utils::RESERVED_RUST_WORDS,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the documentation of the argument.
    pub fn documentation(&self) -> Option<&Documentation> {
        self.documentation.as_ref()
    }

    /// Returns the type of the argument.
    pub fn arg_type(&self) -> &DataVariantRef {
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
    pub fn is_compatible_with(&self, other: &Argument) -> bool {
        self.name == other.name && self.arg_type == other.arg_type
    }
}

impl Argument {
    /// Initializes a new `ArgumentBuilder`.
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
    fn internal_dependencies(&self) -> Vec<&crate::structs::InternalCrate> {
        let mut dependencies = self.arg_type.internal_dependencies();
        if let Some(doc) = &self.documentation {
            dependencies.extend(doc.internal_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}

impl ExternalDependencies for Argument {
    fn external_dependencies(&self) -> Vec<Arc<crate::structs::ExternalCrate>> {
        let mut dependencies = self.arg_type.external_dependencies();
        if let Some(doc) = &self.documentation {
            dependencies.extend(doc.external_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}
