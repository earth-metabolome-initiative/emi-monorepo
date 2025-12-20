//! Submodule defining a struct which represents a rust trait.

mod builder;

pub use builder::TraitDefBuilder;
use common_traits::prelude::Builder;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Ident;

use crate::{
    structs::{
        DataVariantRef, Documentation, InternalCrate, InternalToken, Publicness, Trait,
        TraitVariantRef,
        internal_struct::{Method, WhereClause},
    },
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Struct representing a rust trait.
pub struct TraitDef {
    /// Name of the trait.
    name: String,
    /// Optional path for external traits (None for internal traits).
    path: Option<syn::Path>,
    /// Publicness of the trait.
    publicness: Publicness,
    /// Method definitions.
    methods: Vec<Method>,
    /// Trait documentation.
    documentation: Documentation,
    /// Where statements for the trait.
    where_clauses: Vec<WhereClause>,
    /// Generics for the trait.
    generics: Vec<syn::GenericParam>,
    /// Generic default values (for external traits).
    generic_defaults: Vec<Option<DataVariantRef>>,
    /// Super traits for the trait.
    super_traits: Vec<TraitVariantRef>,
}

impl TraitDef {
    /// Initializes a new `TraitDefBuilder`.
    #[must_use]
    pub fn new() -> TraitDefBuilder {
        TraitDefBuilder::default()
    }

    /// Returns the name of the trait.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the path of the trait (for external traits).
    #[inline]
    #[must_use]
    pub fn path(&self) -> Option<&syn::Path> {
        self.path.as_ref()
    }

    /// Returns the ident of the trait.
    #[inline]
    #[must_use]
    pub fn ident(&self) -> Ident {
        syn::Ident::new(&self.name, proc_macro2::Span::call_site())
    }

    /// Returns the appropriately formatted generics for the trait.
    #[must_use]
    pub fn formatted_generics(&self) -> Option<TokenStream> {
        if self.generics.is_empty() {
            None
        } else {
            let generics = &self.generics;
            Some(quote::quote! { <#(#generics),*> })
        }
    }

    /// Returns the publicness of the module.
    #[inline]
    #[must_use]
    pub fn publicness(&self) -> &Publicness {
        &self.publicness
    }

    /// Returns whether the module is public.
    #[inline]
    #[must_use]
    pub fn is_public(&self) -> bool {
        self.publicness.is_public()
    }

    /// Returns whether all methods have a default implementation.
    pub fn all_methods_have_default_impl(&self) -> bool {
        self.methods.iter().all(Method::has_body)
    }

    /// Returns the methods defined by the trait.
    #[inline]
    #[must_use]
    pub fn methods(&self) -> &Vec<Method> {
        &self.methods
    }

    /// Returns the generics for the trait.
    #[inline]
    #[must_use]
    pub fn generics(&self) -> &[syn::GenericParam] {
        &self.generics
    }

    /// Returns the generic defaults for the trait.
    #[inline]
    #[must_use]
    pub fn generic_defaults(&self) -> &[Option<DataVariantRef>] {
        &self.generic_defaults
    }

    /// Returns the formatted generics without defaults.
    #[must_use]
    pub fn generics_without_defaults(&self) -> impl Iterator<Item = &syn::GenericParam> {
        self.generics
            .iter()
            .zip(self.generic_defaults.iter())
            .filter_map(|(generic, default)| if default.is_none() { Some(generic) } else { None })
    }

    /// Returns the formatted generics with defaults in place.
    #[must_use]
    pub fn generics_with_defaults(&self) -> Option<TokenStream> {
        if self.generics.is_empty() {
            None
        } else {
            let generics_with_defaults =
                self.generics.iter().zip(self.generic_defaults.iter()).map(|(ident, default)| {
                    if let Some(default) = default {
                        let default_with_generics = default.format_with_generics();
                        quote::quote! { #default_with_generics }
                    } else {
                        quote::quote! { #ident }
                    }
                });
            Some(quote::quote! { <#(#generics_with_defaults),*> })
        }
    }

    /// Sets a generic field to the provided `DataVariantRef`.
    #[must_use]
    pub fn set_generic_field(
        &self,
        field: &syn::GenericParam,
        value: DataVariantRef,
    ) -> Option<Self> {
        let mut new_trait = self.clone();
        for (i, generic) in new_trait.generics.iter().enumerate() {
            if generic == field {
                if i >= new_trait.generic_defaults.len() {
                    new_trait.generic_defaults.resize(i + 1, None);
                }
                new_trait.generic_defaults[i] = Some(value);
                return Some(new_trait);
            }
        }
        None
    }

    /// Returns the where clauses, optionally including super-traits.
    #[must_use]
    pub fn where_clauses(&self, include_super_traits: bool) -> Vec<WhereClause> {
        let mut clauses = self.where_clauses.clone();
        if include_super_traits {
            for super_trait in &self.super_traits {
                let super_trait_with_generics = super_trait.format_with_generics();
                clauses.push(
                    WhereClause::new()
                        .left(DataVariantRef::self_type(None))
                        .right(
                            InternalToken::new()
                                .stream(quote::quote! {#super_trait_with_generics})
                                .employed_trait(super_trait.clone())
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                );
            }
        }
        clauses
    }

    /// Returns the formatted where constraints for the trait, including
    /// optionally the super-traits applied to Self.
    #[must_use]
    pub fn formatted_where_constraints(&self, include_super_traits: bool) -> Option<TokenStream> {
        let mut constraints = Vec::new();
        for where_clause in self.where_clauses(include_super_traits) {
            constraints.push(where_clause.to_token_stream());
        }
        if constraints.is_empty() { None } else { Some(quote::quote! { where #(#constraints),* }) }
    }

    /// Returns the requested method by name.
    #[must_use]
    pub fn get_method_by_name(&self, name: &str) -> Option<&Method> {
        self.methods.iter().find(|method| method.name() == name)
    }

    /// Returns whether the trait defines the provided method.
    #[must_use]
    pub fn defines_method(&self, method: &Method) -> bool {
        let Some(curresponding_method) = self.get_method_by_name(method.name()) else {
            return false;
        };
        curresponding_method.is_compatible_with(method)
    }

    /// Returns whether the trait is implemented for a typeless enum.
    pub fn implemented_for_typeless_enum(&self) -> bool {
        let Ok(_reference_trait): Result<Trait, ()> = self.try_into() else {
            return false;
        };
        true
    }

    /// Returns the auto-blanket for the trait, if it can be generated.
    #[must_use]
    pub fn auto_blanket(&self) -> Option<InternalToken> {
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
                        &format!("{letter}{number_of_loops}"),
                        proc_macro2::Span::call_site(),
                    )
                };
                // Check if the tentative generic identifier matches any existing generic
                // parameter
                if self.generics.iter().any(|param| {
                    match param {
                        syn::GenericParam::Type(type_param) => {
                            type_param.ident == tentative_generic
                        }
                        syn::GenericParam::Lifetime(lifetime_param) => {
                            lifetime_param.lifetime.ident == tentative_generic
                        }
                        syn::GenericParam::Const(const_param) => {
                            const_param.ident == tentative_generic
                        }
                    }
                }) {
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

impl InternalDependencies for TraitDef {
    #[inline]
    fn internal_dependencies(&self) -> impl Iterator<Item = &InternalCrate> {
        self.methods
            .iter()
            .flat_map(InternalDependencies::internal_dependencies)
            .chain(self.super_traits.iter().flat_map(InternalDependencies::internal_dependencies))
            .chain(self.where_clauses.iter().flat_map(InternalDependencies::internal_dependencies))
            .chain(self.documentation.internal_dependencies())
    }
}

impl ExternalDependencies for TraitDef {
    #[inline]
    fn external_dependencies(&self) -> impl Iterator<Item = &crate::structs::ExternalCrate> {
        self.methods
            .iter()
            .flat_map(ExternalDependencies::external_dependencies)
            .chain(self.super_traits.iter().flat_map(ExternalDependencies::external_dependencies))
            .chain(self.where_clauses.iter().flat_map(ExternalDependencies::external_dependencies))
            .chain(self.documentation.external_dependencies())
    }
}

impl ToTokens for TraitDef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let publicness = &self.publicness;
        let name = &self.ident();
        let methods = &self.methods;

        let where_clause_tokens = self.formatted_where_constraints(false);

        let documentation = &self.documentation;
        let formatted_generics = self.formatted_generics();

        let maybe_supertraits = if self.super_traits.is_empty() {
            quote::quote! {}
        } else {
            let formatted_super_traits =
                self.super_traits.iter().map(TraitVariantRef::format_with_generics);
            quote::quote! { : #(#formatted_super_traits)+* }
        };

        tokens.extend(quote::quote! {
            #documentation
            #publicness trait #name #formatted_generics #maybe_supertraits #where_clause_tokens {
                #(#methods)*
            }
        });
    }
}

impl From<crate::structs::Trait> for TraitDef {
    fn from(value: crate::structs::Trait) -> Self {
        use common_traits::builder::Builder;
        TraitDef::new().name(value.as_ref()).unwrap().path(value.path()).build().unwrap()
    }
}

impl TraitDef {
    /// Returns a new `Sized` trait definition.
    #[must_use]
    pub fn sized() -> TraitDef {
        use common_traits::builder::Builder;
        TraitDef::new()
            .name(&"Sized")
            .unwrap()
            .path(syn::parse_quote! { std::marker::Sized })
            .build()
            .unwrap()
    }
}
