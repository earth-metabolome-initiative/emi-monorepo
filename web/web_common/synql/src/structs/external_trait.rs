//! Submodule providing enum for trait variant references (internal vs external).

use std::sync::Arc;

use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{
    structs::{
        InternalCrate, TraitDef, TraitImpl, Method, Trait, DataVariantRef,
        external_crate::ExternalTraitRef,
    },
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// Enum representing a trait implemented for some internal data,
/// which may be defined within the workspace or come from an external crate.
pub enum TraitVariantRef {
    /// Variant representing a trait defined within the workspace.
    /// The crate definition is optional for cases where the crate itself
    /// has not yet been defined (e.g., for the current crate).
    Internal(Arc<TraitDef>, Option<Arc<InternalCrate>>),
    /// Variant representing a trait defined within an external crate.
    External(ExternalTraitRef),
}

impl From<ExternalTraitRef> for TraitVariantRef {
    fn from(ext_trait_ref: ExternalTraitRef) -> Self {
        Self::External(ext_trait_ref)
    }
}

impl From<Arc<TraitDef>> for TraitVariantRef {
    fn from(trait_def: Arc<TraitDef>) -> Self {
        Self::Internal(trait_def, None)
    }
}

impl From<Trait> for TraitVariantRef {
    fn from(trait_def: Trait) -> Self {
        Self::External(trait_def.into())
    }
}

impl TraitVariantRef {
    #[inline]
    /// Returns the name of the trait.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            TraitVariantRef::Internal(trait_def, _crate_def) => trait_def.name(),
            TraitVariantRef::External(ext_trait_ref) => ext_trait_ref.name(),
        }
    }

    #[inline]
    /// Returns whether the trait is implemented for typeless enums.
    #[must_use]
    pub fn implemented_for_typeless_enum(&self) -> bool {
        match self {
            TraitVariantRef::Internal(_, _) => false,
            TraitVariantRef::External(ext_trait_ref) => {
                ext_trait_ref.implemented_for_typeless_enum()
            }
        }
    }

    #[inline]
    /// Returns whether the trait defines the provided method.
    #[must_use]
    pub fn defines_method(&self, method: &Method) -> bool {
        match self {
            TraitVariantRef::Internal(trait_def, _crate_def) => trait_def.defines_method(method),
            TraitVariantRef::External(_) => false,
        }
    }

    #[inline]
    /// Returns a reference to the slice of methods defined by the trait.
    #[must_use]
    pub fn methods(&self) -> &[Method] {
        match self {
            TraitVariantRef::Internal(trait_def, _crate_def) => trait_def.methods(),
            TraitVariantRef::External(_) => &[],
        }
    }

    #[inline]
    /// Returns a reference to the method with the provided name, if it exists.
    #[must_use]
    pub fn method(&self, name: &str) -> Option<&Method> {
        self.methods().iter().find(|method| method.name() == name)
    }

    /// Returns the where clauses, optionally including super-traits.
    #[must_use]
    pub fn where_clauses(&self, include_super_traits: bool) -> Vec<crate::structs::WhereClause> {
        match self {
            TraitVariantRef::Internal(trait_def, _crate_def) => {
                trait_def.where_clauses(include_super_traits)
            }
            TraitVariantRef::External(_) => vec![],
        }
    }

    #[inline]
    /// Returns the [`TraitImpl`] struct to implement the trait for the provided
    /// type.
    #[must_use]
    pub fn impl_for_type<'trt>(&'trt self, type_token: &'trt DataVariantRef) -> TraitImpl<'trt> {
        TraitImpl::new(self).for_type(type_token)
    }

    #[inline]
    /// Formats the trait variant reference with generics.
    #[must_use]
    pub fn format_with_generics(&self) -> TokenStream {
        let mut tokens = proc_macro2::TokenStream::new();
        self.to_tokens(&mut tokens);
        match self {
            TraitVariantRef::Internal(trait_def, _crate_def) => {
                tokens.extend(trait_def.formatted_generics());
            }
            TraitVariantRef::External(external) => {
                tokens.extend(external.generics_with_defaults());
            }
        }
        tokens
    }
}

impl ExternalDependencies for TraitVariantRef {
    #[inline]
    fn external_dependencies(&self) -> impl Iterator<Item = &crate::structs::ExternalCrate> {
        let vec: Vec<&crate::structs::ExternalCrate> = match self {
            TraitVariantRef::Internal(_, _) => vec![],
            TraitVariantRef::External(ext_trait_ref) => {
                ext_trait_ref.external_dependencies().collect()
            }
        };
        vec.into_iter()
    }
}

impl InternalDependencies for TraitVariantRef {
    #[inline]
    fn internal_dependencies(&self) -> impl Iterator<Item = &InternalCrate> {
        let vec: Vec<&InternalCrate> = match self {
            TraitVariantRef::Internal(_, crate_def) => {
                crate_def.iter().map(AsRef::as_ref).collect()
            }
            TraitVariantRef::External(external) => external.internal_dependencies().collect(),
        };
        vec.into_iter()
    }
}

impl ToTokens for TraitVariantRef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            TraitVariantRef::Internal(trait_def, crate_def) => {
                let trait_ident = trait_def.ident();
                let crate_name = crate_def.as_ref().map_or_else(
                    || quote::quote! { crate },
                    |crate_def| {
                        let crate_ident = crate_def.ident();
                        quote::quote! { #crate_ident }
                    },
                );
                tokens.extend(quote::quote! {
                    #crate_name::prelude::#trait_ident
                });
            }
            TraitVariantRef::External(ext_trait_ref) => {
                tokens.extend(quote::quote! {
                    #ext_trait_ref
                });
            }
        }
    }
}
