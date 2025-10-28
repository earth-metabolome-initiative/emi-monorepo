//! Submodule defining a struct which represents a rust module.

mod builder;

pub use builder::InternalTraitBuilder;
use common_traits::prelude::Builder;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Ident;

use crate::{
    structs::{
        InternalCrate, InternalToken, Publicness,
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
    /// Super traits for the trait.
    super_traits: Vec<InternalToken<'data>>,
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

    /// Returns the appropriately formatted generics for the trait.
    pub fn formatted_generics(&self) -> Option<TokenStream> {
        if self.generics.is_empty() {
            None
        } else {
            let generics = &self.generics;
            Some(quote::quote! { <#(#generics),*> })
        }
    }

    /// Returns the publicness of the module.
    pub fn publicness(&self) -> &Publicness {
        &self.publicness
    }

    /// Returns whether the module is public.
    pub fn is_public(&self) -> bool {
        self.publicness.is_public()
    }

    /// Returns whether all methods have a default implementation.
    pub fn all_methods_have_default_impl(&self) -> bool {
        self.methods.iter().all(|method| method.has_body())
    }

    /// Returns the formatted where constraints for the trait, including
    /// optionally the super-traits applied to Self.
    pub fn formatted_where_constraints(&self, include_super_traits: bool) -> Option<TokenStream> {
        let mut constraints = Vec::new();
        for where_clause in &self.where_statements {
            constraints.push(where_clause.to_token_stream());
        }
        if include_super_traits {
            for super_trait in &self.super_traits {
                constraints.push(quote::quote! { Self: #super_trait });
            }
        }
        if constraints.is_empty() { None } else { Some(quote::quote! { where #(#constraints),* }) }
    }

    /// Returns the auto-blanket for the trait, if it can be generated.
    pub fn auto_blanket(&self) -> Option<InternalToken<'data>> {
        if !self.all_methods_have_default_impl() {
            return None;
        }

        // We iterate over the alphabet to identify a letter which does not
        // appear in the generics of the trait. This letter will be used as the
        // generic for the blanket impl. Additionally, we loop until we find a
        // letter which is not used in any of the where clauses.
        let alphabet = 'A'..='Z';
        let mut number_of_loops = 0;

        let free_generic = 'outer: loop {
            for letter in alphabet.clone() {
                let tentative_generic = if number_of_loops == 0 {
                    syn::Ident::new(&letter.to_string(), proc_macro2::Span::call_site())
                } else {
                    syn::Ident::new(
                        &format!("{}{}", letter, number_of_loops),
                        proc_macro2::Span::call_site(),
                    )
                };
                if self.generics.contains(&tentative_generic) {
                    continue;
                }
                break 'outer tentative_generic;
            }
            number_of_loops += 1;
        };

        let generics = &self.generics;
        let where_clause_tokens = self.formatted_where_constraints(true);
        let formatted_generics = self.formatted_generics();
        let trait_ident = self.ident();

        Some(InternalToken::new().public().stream(quote::quote!{
            impl<#free_generic, #(#generics),*> #trait_ident #formatted_generics for #free_generic #where_clause_tokens {}
        }).build().unwrap())
    }
}

impl<'data> InternalDependencies<'data> for InternalTrait<'data> {
    fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        let mut dependencies = Vec::new();
        for method in &self.methods {
            dependencies.extend(method.internal_dependencies());
        }
        for super_trait in &self.super_traits {
            dependencies.extend(super_trait.internal_dependencies());
        }
        for where_clause in &self.where_statements {
            dependencies.extend(where_clause.internal_dependencies());
        }
        for super_trait in &self.super_traits {
            dependencies.extend(super_trait.internal_dependencies());
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
        for super_trait in &self.super_traits {
            dependencies.extend(super_trait.external_dependencies());
        }
        for where_clause in &self.where_statements {
            dependencies.extend(where_clause.external_dependencies());
        }
        for super_trait in &self.super_traits {
            dependencies.extend(super_trait.external_dependencies());
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

        let where_clause_tokens = self.formatted_where_constraints(false);

        let documentation = &self.documentation;
        let documentation = quote::quote! {
            #[doc = #documentation]
        };

        let formatted_generics = self.formatted_generics();

        let maybe_supertraits = if self.super_traits.is_empty() {
            quote::quote! {}
        } else {
            let super_traits = &self.super_traits;
            quote::quote! { : #(#super_traits)+* }
        };

        tokens.extend(quote::quote! {
            #documentation
            #publicness trait #name #formatted_generics #maybe_supertraits #where_clause_tokens {
                #(#methods)*
            }
        });
    }
}
