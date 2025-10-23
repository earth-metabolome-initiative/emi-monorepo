//! Submodule providing the `Argument` struct which represents a rust method
//! argument.

use quote::ToTokens;

use crate::{
    structs::internal_data::DataVariantRef,
    traits::{ExternalDependencies, InternalDependencies},
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
    documentation: Option<String>,
}

impl ToTokens for Argument<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if self.name == "self" {
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
        self.arg_type.internal_dependencies()
    }
}

impl<'data> ExternalDependencies<'data> for Argument<'data> {
    fn external_dependencies(&self) -> Vec<&crate::structs::ExternalCrate<'data>> {
        self.arg_type.external_dependencies()
    }
}
