//! Submodule providing a struct defining a trait available in an external
//! crate.

mod builder;
use std::{fmt::Debug, hash::Hash, sync::Arc};

use builder::ExternalTraitBuilder;
use quote::ToTokens;

use crate::{
    structs::{
        DataVariantRef, InternalCrate, InternalTrait, Method, Trait, TraitImpl,
        external_crate::ExternalTraitRef,
    },
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Clone)]
/// Struct defining a trait available in an external crate.
pub struct ExternalTrait {
    /// The name of the trait.
    name: String,
    /// The [`syn::Path`](syn::Path) representing the trait
    /// within the external crate.
    path: syn::Path,
}

impl ToTokens for ExternalTrait {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let path = &self.path;
        tokens.extend(quote::quote! {
            #path
        });
    }
}

impl From<Trait> for ExternalTrait {
    fn from(value: Trait) -> Self {
        ExternalTrait { name: value.as_ref().to_owned(), path: value.path() }
    }
}

impl TryFrom<&ExternalTrait> for Trait {
    type Error = ();

    fn try_from(value: &ExternalTrait) -> Result<Self, Self::Error> {
        match value.name.as_str() {
            "Clone" => Ok(Trait::Clone),
            "Copy" => Ok(Trait::Copy),
            "Debug" => Ok(Trait::Debug),
            "Default" => Ok(Trait::Default),
            "PartialEq" => Ok(Trait::PartialEq),
            "Eq" => Ok(Trait::Eq),
            "PartialOrd" => Ok(Trait::PartialOrd),
            "Ord" => Ok(Trait::Ord),
            "Hash" => Ok(Trait::Hash),
            _ => Err(()),
        }
    }
}

impl Debug for ExternalTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExternalTrait")
            .field("name", &self.name)
            .field("path", &self.path.to_token_stream().to_string())
            .finish()
    }
}

impl PartialEq for ExternalTrait {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.path.to_token_stream().to_string() == other.path.to_token_stream().to_string()
    }
}

impl Eq for ExternalTrait {}

impl Hash for ExternalTrait {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.path.to_token_stream().to_string().hash(state);
    }
}

impl PartialOrd for ExternalTrait {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ExternalTrait {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.name.cmp(&other.name) {
            std::cmp::Ordering::Equal => {
                self.path
                    .to_token_stream()
                    .to_string()
                    .cmp(&other.path.to_token_stream().to_string())
            }
            ord => ord,
        }
    }
}

unsafe impl Send for ExternalTrait {}
unsafe impl Sync for ExternalTrait {}

impl ExternalTrait {
    /// Inizializes a new `ExternalTraitBuilder`.
    pub fn new() -> ExternalTraitBuilder {
        ExternalTraitBuilder::default()
    }

    /// Returns the name of the trait.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the [`syn::Path`](syn::Path) representing the trait
    /// within the external crate.
    pub fn path(&self) -> &syn::Path {
        &self.path
    }

    /// Returns whether the trait is implemented for typeless enums.
    pub fn implemented_for_typeless_enum(&self) -> bool {
        if self.path.to_token_stream().to_string() == quote::quote! { serde::Serialize }.to_string()
        {
            return true;
        }
        if self.path.to_token_stream().to_string()
            == quote::quote! { serde::Deserialize }.to_string()
        {
            return true;
        }

        let Ok(trait_variant) = Trait::try_from(self) else {
            return false;
        };
        matches!(
            trait_variant,
            Trait::Clone
                | Trait::Debug
                | Trait::Copy
                | Trait::PartialEq
                | Trait::Eq
                | Trait::Hash
                | Trait::PartialOrd
                | Trait::Ord
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Enum representing a trait implemented for some internal data,
/// which may be defined within the workspace or come from an external crate.
pub enum TraitVariantRef {
    /// Variant representing a trait defined within the workspace.
    /// The crate definition is optional for cases where the crate itself
    /// has not yet been defined (e.g., for the current crate).
    Internal(Arc<InternalTrait>, Option<Arc<InternalCrate>>),
    /// Variant representing a trait defined within an external crate.
    External(ExternalTraitRef),
}

impl From<ExternalTraitRef> for TraitVariantRef {
    fn from(ext_trait_ref: ExternalTraitRef) -> Self {
        Self::External(ext_trait_ref)
    }
}

impl From<Arc<InternalTrait>> for TraitVariantRef {
    fn from(trait_def: Arc<InternalTrait>) -> Self {
        Self::Internal(trait_def, None)
    }
}

impl From<Trait> for TraitVariantRef {
    fn from(trait_def: Trait) -> Self {
        Self::External(trait_def.into())
    }
}

impl TraitVariantRef {
    /// Returns the name of the trait.
    pub fn name(&self) -> &str {
        match self {
            TraitVariantRef::Internal(trait_def, _crate_def) => trait_def.name(),
            TraitVariantRef::External(ext_trait_ref) => ext_trait_ref.name(),
        }
    }

    /// Returns whether the trait is implemented for typeless enums.
    pub fn implemented_for_typeless_enum(&self) -> bool {
        match self {
            TraitVariantRef::Internal(_, _) => false,
            TraitVariantRef::External(ext_trait_ref) => {
                ext_trait_ref.implemented_for_typeless_enum()
            }
        }
    }

    /// Returns whether the trait defines the provided method.
    pub fn defines_method(&self, method: &Method) -> bool {
        match self {
            TraitVariantRef::Internal(trait_def, _crate_def) => trait_def.defines_method(method),
            TraitVariantRef::External(_) => false,
        }
    }

    /// Returns a reference to the slice of methods defined by the trait.
    pub fn methods(&self) -> &[Method] {
        match self {
            TraitVariantRef::Internal(trait_def, _crate_def) => trait_def.methods(),
            TraitVariantRef::External(_) => &[],
        }
    }

    /// Returns a reference to the method with the provided name, if it exists.
    pub fn method(&self, name: &str) -> Option<&Method> {
        self.methods().iter().find(|method| method.name() == name)
    }

    /// Returns the [`TraitImpl`] struct to implement the trait for the provided
    /// type.
    pub fn impl_for_type<'trt>(&'trt self, type_token: &'trt DataVariantRef) -> TraitImpl<'trt> {
        TraitImpl::new(self).for_type(type_token)
    }
}

impl ExternalDependencies for TraitVariantRef {
    fn external_dependencies(&self) -> Vec<Arc<crate::structs::ExternalCrate>> {
        match self {
            TraitVariantRef::Internal(_, _) => vec![],
            TraitVariantRef::External(ext_trait_ref) => ext_trait_ref.external_dependencies(),
        }
    }
}

impl InternalDependencies for TraitVariantRef {
    fn internal_dependencies(&self) -> Vec<&InternalCrate> {
        match self {
            TraitVariantRef::Internal(_, crate_def) => {
                crate_def.into_iter().map(AsRef::as_ref).collect()
            }
            TraitVariantRef::External(_) => vec![],
        }
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
