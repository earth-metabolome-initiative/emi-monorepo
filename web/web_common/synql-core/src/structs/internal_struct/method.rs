//! Submodule providing the `Method` struct which represents a rust method.

mod argument;
mod builder;
mod where_clause;

pub use argument::Argument;
pub use builder::MethodBuilder;
use quote::ToTokens;
pub use where_clause::WhereClause;

use crate::{
    structs::{Documentation, InternalToken, Publicness, internal_data::DataVariantRef},
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Struct representing a rust method.
pub struct Method {
    /// Arguments of the method.
    arguments: Vec<Argument>,
    /// Name of the method.
    name: String,
    /// Publicness of the method.
    publicness: Publicness,
    /// The body of the method.
    body: Option<InternalToken>,
    /// Whether the method is asynchronous.
    async_method: bool,
    /// The return type of the method.
    return_type: Option<DataVariantRef>,
    /// Documentation of the method.
    documentation: Documentation,
    /// Error documentation of the method.
    error_documentations: Vec<Documentation>,
    /// Generics of the method.
    generics: Vec<syn::GenericParam>,
    /// Where clauses of the method.
    where_clauses: Vec<WhereClause>,
    /// Whether the method is a trait implementation method.
    is_trait_implementation: bool,
}

impl Method {
    /// Returns the name of the method.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the ident of the method.
    #[inline]
    pub fn ident(&self) -> syn::Ident {
        syn::Ident::new(&self.name, proc_macro2::Span::call_site())
    }

    /// Returns a reference to the body of the method.
    #[inline]
    pub fn body(&self) -> Option<&InternalToken> {
        self.body.as_ref()
    }

    /// Returns an iterator over the where clauses of the method.
    pub fn where_clauses(&self) -> impl Iterator<Item = &WhereClause> {
        self.where_clauses.iter()
    }

    /// Returns whether the method has a body.
    #[inline]
    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    /// Returns whether the method is a trait implementation method.
    #[inline]
    pub fn is_trait_implementation(&self) -> bool {
        self.is_trait_implementation
    }

    /// Sets as a trait implementation method.
    pub fn set_as_trait_implementation(&mut self) {
        self.is_trait_implementation = true;
    }
}

impl Method {
    /// Initializes a new `MethodBuilder`.
    pub fn new() -> MethodBuilder {
        MethodBuilder::default()
    }

    /// Iterates over the arguments of the method.
    pub fn arguments(&self) -> impl Iterator<Item = &Argument> {
        self.arguments.iter()
    }

    /// Returns the return type of the method.
    pub fn return_type(&self) -> Option<&DataVariantRef> {
        self.return_type.as_ref()
    }

    /// Returns an iterator over the non-self arguments of the method.
    pub fn non_self_arguments(&self) -> impl Iterator<Item = &Argument> {
        self.arguments.iter().filter(|arg| !arg.is_self())
    }

    /// Returns whether the method has non-self arguments.
    pub fn has_non_self_arguments(&self) -> bool {
        self.non_self_arguments().next().is_some()
    }

    /// Returns the method signature as a string.
    pub fn signature(&self) -> String {
        let args = self
            .arguments
            .iter()
            .map(|arg| arg.to_token_stream().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let return_type = if let Some(ret_type) = &self.return_type {
            format!(" -> {}", ret_type.to_token_stream().to_string())
        } else {
            String::new()
        };
        let generics = if self.generics.is_empty() {
            String::new()
        } else {
            format!(
                "<{}>",
                self.generics
                    .iter()
                    .map(|ident| ident.to_token_stream().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        format!("fn {}{generics}({args}){return_type}", self.name)
    }

    /// Returns whether the method can fail, (i.e. has a return type of
    /// `Result`).
    pub fn can_fail(&self) -> bool {
        self.return_type.as_ref().map_or(false, |ret_type| ret_type.is_result())
    }

    /// Returns whether the method is compatible with the provided method
    /// signature.
    pub fn is_compatible_with(&self, other: &Method) -> bool {
        self.arguments.iter().zip(other.arguments.iter()).all(|(a, b)| a.is_compatible_with(b))
            && self.async_method == other.async_method
            && self.return_type == other.return_type
            && self.generics == other.generics
            && self.where_clauses == other.where_clauses
    }
}

impl ToTokens for Method {
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
            let ty_tokens = return_type.format_with_generics();
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

        let mut documentation = Vec::new();

        if self.has_non_self_arguments() {
            documentation.push(String::default());
            documentation.push(" # Arguments".to_string());
            for arg in self.non_self_arguments() {
                documentation.push(format!(
                    " * `{}` - {}",
                    arg.name(),
                    arg.documentation().map(|d| d.documentation()).unwrap_or_default()
                ));
            }
        }

        if !self.error_documentations.is_empty() {
            documentation.push(String::default());
            documentation.push("# Errors".to_string());
            for error_doc in &self.error_documentations {
                documentation.push(format!(" * {}", error_doc.documentation()));
            }
        }

        let main_documentation = if self.is_trait_implementation() {
            None
        } else {
            let mut main_documentation = self.documentation.clone();
            main_documentation.extend(&documentation.join("\n"));
            Some(main_documentation)
        };

        // Automatically determine if method should have #[inline] attribute
        // A method should be inlined if it has a body and the body is small (simple
        // delegation or trivial logic)
        let should_inline = if let Some(body) = &self.body {
            let body_str = body.to_token_stream().to_string();
            // Parse and format the body to get proper line count
            // We parse as a file since prettyplease::unparse only works with syn::File
            let formatted_body = match syn::parse_file(&body_str) {
                Ok(syntax_tree) => prettyplease::unparse(&syntax_tree),
                Err(_) => body_str.clone(),
            };
            // Consider a method small enough for inlining if:
            // - Body has 5 or fewer non-empty lines
            // - Or unformatted body is less than 200 characters
            let line_count = formatted_body.lines().filter(|line| !line.trim().is_empty()).count();
            line_count <= 5 || body_str.len() < 200
        } else {
            false
        };

        // Automatically determine if method should have #[must_use] attribute
        // A method should have must_use if it:
        // - Returns a value (not unit type)
        // - Has a body (is not a trait method declaration)
        // - Does NOT return Result or Option (these types already have #[must_use])
        // - Does NOT take &mut self (mutation methods may return secondary values that
        //   don't need to be used)
        let should_must_use = self.return_type.is_some()
            && !self.is_trait_implementation()
            && !self.return_type.as_ref().map_or(false, |rt| rt.is_option() || rt.is_result())
            && !self.arguments.iter().any(|arg| arg.is_mut_self());

        let inline_attr = should_inline.then(|| quote::quote! { #[inline] });

        let must_use_attr = should_must_use.then(|| quote::quote! { #[must_use] });

        tokens.extend(quote::quote! {
            #main_documentation
            #inline_attr
            #must_use_attr
			#pubness_tokens #async_tokens fn #name_ident #formatted_generics (#(#arguments_tokens),*) #return_type_tokens #formatted_where #formatted_body
		});
    }
}

impl InternalDependencies for Method {
    #[inline]
    fn internal_dependencies(&self) -> impl Iterator<Item = &crate::structs::InternalCrate> {
        self.return_type
            .iter()
            .flat_map(|return_type| return_type.internal_dependencies())
            .chain(self.arguments.iter().flat_map(|arg| arg.internal_dependencies()))
            .chain(
                self.where_clauses
                    .iter()
                    .flat_map(|where_clause| where_clause.internal_dependencies()),
            )
            .chain(self.documentation.internal_dependencies())
            .chain(
                self.error_documentations
                    .iter()
                    .flat_map(|error_doc| error_doc.internal_dependencies()),
            )
    }
}

impl ExternalDependencies for Method {
    #[inline]
    fn external_dependencies(&self) -> impl Iterator<Item = &crate::structs::ExternalCrate> {
        self.return_type
            .iter()
            .flat_map(|return_type| return_type.external_dependencies())
            .chain(self.arguments.iter().flat_map(|arg| arg.external_dependencies()))
            .chain(
                self.where_clauses
                    .iter()
                    .flat_map(|where_clause| where_clause.external_dependencies()),
            )
            .chain(self.documentation.external_dependencies())
            .chain(
                self.error_documentations
                    .iter()
                    .flat_map(|error_doc| error_doc.external_dependencies()),
            )
    }
}
