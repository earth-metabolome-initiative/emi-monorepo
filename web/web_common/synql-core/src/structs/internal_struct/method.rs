//! Submodule providing the `Method` struct which represents a rust method.

mod argument;
mod builder;
mod where_clause;
pub use argument::Argument;
pub use builder::MethodBuilder;
use quote::ToTokens;
use syn::Ident;
pub use where_clause::WhereClause;

use crate::{
    structs::{InternalToken, Publicness, internal_data::DataVariantRef},
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a rust method.
pub struct Method<'data> {
    /// Arguments of the method.
    arguments: Vec<Argument<'data>>,
    /// Name of the method.
    name: String,
    /// Publicness of the method.
    publicness: Publicness,
    /// The body of the method.
    body: Option<InternalToken<'data>>,
    /// Whether the method is asynchronous.
    async_method: bool,
    /// The return type of the method.
    return_type: Option<DataVariantRef<'data>>,
    /// Documentation of the method.
    documentation: String,
    /// Error documentation of the method.
    error_documentation: Option<String>,
    /// Generics of the method.
    generics: Vec<Ident>,
    /// Where clauses of the method.
    where_clauses: Vec<WhereClause<'data>>,
}

impl Method<'_> {
    /// Returns the name of the method.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the method has a body.
    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }
}

impl<'data> Method<'data> {
    /// Initializes a new `MethodBuilder`.
    pub fn new() -> MethodBuilder<'data> {
        MethodBuilder::default()
    }

    /// Returns an iterator over the non-self arguments of the method.
    pub fn non_self_arguments(&self) -> impl Iterator<Item = &Argument<'data>> {
        self.arguments.iter().filter(|arg| !arg.is_self())
    }

    /// Returns whether the method has non-self arguments.
    pub fn has_non_self_arguments(&self) -> bool {
        self.non_self_arguments().next().is_some()
    }

    /// Returns whether the method can fail, (i.e. has a return type of
    /// `Result`).
    pub fn can_fail(&self) -> bool {
        self.return_type.as_ref().map_or(false, |ret_type| ret_type.is_result())
    }
}

impl ToTokens for Method<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name_ident = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        let pubness_tokens = self.publicness.to_token_stream();
        let async_tokens = if self.async_method {
            quote::quote! { async }
        } else {
            quote::quote! {}
        };
        let arguments_tokens =
            self.arguments.iter().map(|arg| arg.to_token_stream()).collect::<Vec<_>>();
        let return_type_tokens = if let Some(return_type) = &self.return_type {
            let ty_tokens = return_type.to_token_stream();
            quote::quote! { -> #ty_tokens }
        } else {
            quote::quote! {}
        };
        let body_tokens = self.body.to_token_stream();

        let formatted_generics = if !self.generics.is_empty() {
            let generics_idents = &self.generics;
            quote::quote! { <#(#generics_idents),*> }
        } else {
            quote::quote! {}
        };

        let formatted_body = if let Some(_) = &self.body {
            quote::quote! { { #body_tokens } }
        } else {
            quote::quote! { ; }
        };

        let formatted_where = if self.where_clauses.is_empty() {
            quote::quote! {}
        } else {
            let where_tokens = &self.where_clauses;
            quote::quote! { where #(#where_tokens),* }
        };

        let mut documentation = vec![self.documentation.clone()];

        if self.has_non_self_arguments() {
            documentation.push(String::default());
            documentation.push("# Arguments".to_string());
            for arg in self.non_self_arguments() {
                documentation.push(format!(
                    " * `{}` - {}",
                    arg.name(),
                    arg.documentation().unwrap_or_default()
                ));
            }
        }

        if let Some(error_doc) = &self.error_documentation {
            documentation.push(String::default());
            documentation.push("# Errors".to_string());
            documentation.push(error_doc.clone());
        }

        tokens.extend(quote::quote! {
            #(#[doc = #documentation])*
			#pubness_tokens #async_tokens fn #name_ident #formatted_generics (#(#arguments_tokens),*) #return_type_tokens #formatted_where #formatted_body
		});
    }
}

impl<'data> InternalDependencies<'data> for Method<'data> {
    fn internal_dependencies(&self) -> Vec<&crate::structs::InternalCrate<'data>> {
        let mut dependencies = Vec::new();
        if let Some(return_type) = &self.return_type {
            dependencies.extend(return_type.internal_dependencies());
        }
        for arg in &self.arguments {
            dependencies.extend(arg.internal_dependencies());
        }
        for where_clause in &self.where_clauses {
            dependencies.extend(where_clause.internal_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}

impl<'data> ExternalDependencies<'data> for Method<'data> {
    fn external_dependencies(&self) -> Vec<&crate::structs::ExternalCrate<'data>> {
        let mut dependencies = Vec::new();
        if let Some(return_type) = &self.return_type {
            dependencies.extend(return_type.external_dependencies());
        }
        for arg in &self.arguments {
            dependencies.extend(arg.external_dependencies());
        }
        for where_clause in &self.where_clauses {
            dependencies.extend(where_clause.external_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}
