//! Submodule defining a struct which represents a rust module.

mod builder;

pub use builder::InternalTraitBuilder;
use quote::ToTokens;
use syn::Ident;

use crate::{
    structs::{
        InternalCrate, Publicness,
        internal_struct::{Method, WhereClause},
    },
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a rust trait.
pub struct InternalTrait<'data> {
    /// Name of the trait.
    name: String,
    /// Publicness of the trait.
    publicness: Publicness,
    /// Method definitions.
    methods: Vec<Method<'data>>,
    /// Trait documentation.
    documentation: String,
    /// Where statements for the trait.
    where_statements: Vec<WhereClause<'data>>,
    /// Generics for the trait.
    generics: Vec<Ident>,
}

impl<'data> InternalTrait<'data> {
    /// Initializes a new `InternalTraitBuilder`.
    pub fn new() -> InternalTraitBuilder<'data> {
        InternalTraitBuilder::default()
    }

    /// Returns the name of the module.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the ident of the module.
    pub fn ident(&self) -> Ident {
        syn::Ident::new(&self.name, proc_macro2::Span::call_site())
    }

    /// Returns the publicness of the module.
    pub fn publicness(&self) -> &Publicness {
        &self.publicness
    }

    /// Returns whether the module is public.
    pub fn is_public(&self) -> bool {
        self.publicness.is_public()
    }
}

impl<'data> InternalDependencies<'data> for InternalTrait<'data> {
    fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        let mut dependencies = Vec::new();
        for method in &self.methods {
            dependencies.extend(method.internal_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}

impl<'data> ExternalDependencies<'data> for InternalTrait<'data> {
    fn external_dependencies(&self) -> Vec<&crate::structs::ExternalCrate<'data>> {
        let mut dependencies = Vec::new();
        for method in &self.methods {
            dependencies.extend(method.external_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}

impl ToTokens for InternalTrait<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let publicness = &self.publicness;
        let name = &self.ident();
        let methods = &self.methods;
        let where_clauses = &self.where_statements;

        let where_clause_tokens = if where_clauses.is_empty() {
            quote::quote! {}
        } else {
            let clauses = where_clauses.iter();
            quote::quote! {
                where #(#clauses),*
            }
        };

        let documentation = &self.documentation;
        let documentation = quote::quote! {
            #[doc = #documentation]
        };

        let formatted_generics = if self.generics.is_empty() {
            quote::quote! {}
        } else {
            let generics = &self.generics;
            quote::quote! { <#(#generics),*> }
        };

        tokens.extend(quote::quote! {
            #documentation
            #publicness trait #name #formatted_generics #where_clause_tokens {
                #(#methods)*
            }
        });
    }
}
