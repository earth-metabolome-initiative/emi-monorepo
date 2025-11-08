//! Submodule providing a struct defining the type required by some type found
//! in the postgres database schema.

mod builder;
mod traits_mask;
use std::{fmt::Debug, hash::Hash, sync::Arc};

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;
pub use traits_mask::Trait;

use crate::{
    structs::{
        DataVariantRef, external_crate::ExternalTraitRef, external_trait::TraitVariantRef,
        external_type::builder::ExternalTypeBuilder,
    },
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Clone)]
/// Struct defining the type required by some type found in the postgres
/// database schema.
pub struct ExternalType {
    /// The diesel type defined within the crate compatible with the given
    /// postgres type.
    diesel_type: Option<syn::Type>,
    /// The rust type defined within the crate compatible with the given
    /// postgres type.
    rust_type: syn::Type,
    /// The postgres types which are compatible with the diesel and rust types
    /// defined within the crate.
    postgres_types: Vec<&'static str>,
    /// The traits supported by the current type.
    traits: traits_mask::TraitsMask,
    /// External traits implemented by the type.
    external_traits: Vec<ExternalTraitRef>,
    /// Generic parameters of the type.
    generics: Vec<Ident>,
    /// Default values for the generic parameters of the type.
    generic_defaults: Vec<Option<DataVariantRef>>,
}

impl PartialEq for ExternalType {
    fn eq(&self, other: &Self) -> bool {
        self.diesel_type.to_token_stream().to_string()
            == other.diesel_type.to_token_stream().to_string()
            && self.rust_type.to_token_stream().to_string()
                == other.rust_type.to_token_stream().to_string()
            && self.postgres_types == other.postgres_types
            && self.traits == other.traits
            && self.external_traits == other.external_traits
            && self.generics == other.generics
            && self
                .generic_defaults
                .iter()
                .zip(other.generic_defaults.iter())
                .all(|(a, b)| a.to_token_stream().to_string() == b.to_token_stream().to_string())
    }
}

impl Eq for ExternalType {}

impl PartialOrd for ExternalType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ExternalType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let diesel_cmp = self
            .diesel_type
            .to_token_stream()
            .to_string()
            .cmp(&other.diesel_type.to_token_stream().to_string());
        if diesel_cmp != std::cmp::Ordering::Equal {
            return diesel_cmp;
        }

        let rust_cmp = self
            .rust_type
            .to_token_stream()
            .to_string()
            .cmp(&other.rust_type.to_token_stream().to_string());
        if rust_cmp != std::cmp::Ordering::Equal {
            return rust_cmp;
        }

        let pg_types_cmp = self.postgres_types.cmp(&other.postgres_types);
        if pg_types_cmp != std::cmp::Ordering::Equal {
            return pg_types_cmp;
        }

        let traits_cmp = self.traits.cmp(&other.traits);
        if traits_cmp != std::cmp::Ordering::Equal {
            return traits_cmp;
        }

        let generics_cmp = self.generics.cmp(&other.generics);
        if generics_cmp != std::cmp::Ordering::Equal {
            return generics_cmp;
        }

        for (self_def, other_def) in self.generic_defaults.iter().zip(other.generic_defaults.iter())
        {
            let def_cmp = self_def
                .to_token_stream()
                .to_string()
                .cmp(&other_def.to_token_stream().to_string());
            if def_cmp != std::cmp::Ordering::Equal {
                return def_cmp;
            }
        }

        self.traits.cmp(&other.traits)
    }
}

impl Hash for ExternalType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.diesel_type.to_token_stream().to_string().hash(state);
        self.rust_type.to_token_stream().to_string().hash(state);
        self.postgres_types.hash(state);
        self.traits.hash(state);
        self.generics.hash(state);
        for def in &self.generic_defaults {
            def.to_token_stream().to_string().hash(state);
        }
    }
}

unsafe impl Send for ExternalType {}
unsafe impl Sync for ExternalType {}

impl Debug for ExternalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExternalType")
            .field("diesel_type", &self.diesel_type.to_token_stream().to_string())
            .field("rust_type", &self.rust_type.to_token_stream().to_string())
            .field("postgres_types", &self.postgres_types)
            .field("traits", &self.traits)
            .field("external_traits", &self.external_traits)
            .field("generics", &self.generics)
            .finish()
    }
}

impl ExternalType {
    /// Inizializes a new `ExternalTypeBuilder`.
    pub fn new() -> ExternalTypeBuilder {
        ExternalTypeBuilder::default()
    }

    /// Returns the diesel type defined within the crate compatible with the
    /// given postgres type.
    pub fn diesel_type(&self) -> Option<&syn::Type> {
        self.diesel_type.as_ref()
    }

    /// Returns the rust type defined within the crate compatible with the given
    /// postgres type.
    pub fn rust_type(&self) -> &syn::Type {
        &self.rust_type
    }

    /// Returns a reference over the postgres types which are compatible with
    /// the diesel and rust types defined within the crate.
    pub fn postgres_types(&self) -> &[&'static str] {
        &self.postgres_types
    }

    /// Returns whether the type is a `Unit` type.
    pub fn is_unit(&self) -> bool {
        self.rust_type.to_token_stream().to_string() == "()"
    }

    /// Returns whether the Rust type associated with the current `ExternalType`
    /// supports the given trait.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait to check support for.
    pub fn supports(&self, trait_ref: &TraitVariantRef) -> bool {
        match trait_ref {
            TraitVariantRef::External(ext_trait_ref) => {
                if self.external_traits.contains(ext_trait_ref) {
                    return true;
                }
                if let Ok(r#trait) = ext_trait_ref.external_trait().try_into() {
                    return self.traits.supports(r#trait);
                }
            }
            TraitVariantRef::Internal(_, _) => {}
        }
        false
    }

    /// Returns whether the current `ExternalType` is compatible with the given
    /// postgres type.
    ///
    /// # Arguments
    /// * `postgres_type` - The postgres type to check compatibility with.
    pub fn is_compatible_with(&self, postgres_type: &str) -> bool {
        self.postgres_types.iter().any(|t| t.eq_ignore_ascii_case(postgres_type))
    }

    /// Casts a value to the external type.
    pub(crate) fn cast(&self, value: &str) -> Result<proc_macro2::TokenStream, syn::Error> {
        let string_rust_type = self.rust_type.to_token_stream().to_string();

        Ok(match string_rust_type.as_str() {
            "String" | "str" => {
                quote! { #value }
            }
            "bool" => {
                let casted: bool = value.parse().map_err(|_| {
                    syn::Error::new_spanned(
                        self.rust_type.to_token_stream(),
                        format!("Cannot cast value to external type: {:?}", self),
                    )
                })?;
                quote! { #casted }
            }
            "i16" => {
                let casted: i16 = value.parse().map_err(|_| {
                    syn::Error::new_spanned(
                        self.rust_type.to_token_stream(),
                        format!("Cannot cast value to external type: {:?}", self),
                    )
                })?;
                quote! { #casted }
            }
            "u16" => {
                let casted: u16 = value.parse().map_err(|_| {
                    syn::Error::new_spanned(
                        self.rust_type.to_token_stream(),
                        format!("Cannot cast value to external type: {:?}", self),
                    )
                })?;
                quote! { #casted }
            }
            "i32" => {
                let casted: i32 = value.parse().map_err(|_| {
                    syn::Error::new_spanned(
                        self.rust_type.to_token_stream(),
                        format!("Cannot cast value to external type: {:?}", self),
                    )
                })?;
                quote! { #casted }
            }
            "u32" => {
                let casted: u32 = value.parse().map_err(|_| {
                    syn::Error::new_spanned(
                        self.rust_type.to_token_stream(),
                        format!("Cannot cast value to external type: {:?}", self),
                    )
                })?;
                quote! { #casted }
            }
            "i64" => {
                let casted: i64 = value.parse().map_err(|_| {
                    syn::Error::new_spanned(
                        self.rust_type.to_token_stream(),
                        format!("Cannot cast value to external type: {:?}", self),
                    )
                })?;
                quote! { #casted }
            }
            "u64" => {
                let casted: u64 = value.parse().map_err(|_| {
                    syn::Error::new_spanned(
                        self.rust_type.to_token_stream(),
                        format!("Cannot cast value to external type: {:?}", self),
                    )
                })?;
                quote! { #casted }
            }
            "f32" => {
                let casted: f32 = value.parse().map_err(|_| {
                    syn::Error::new_spanned(
                        self.rust_type.to_token_stream(),
                        format!("Cannot cast value to external type: {:?}", self),
                    )
                })?;
                quote! { #casted }
            }
            "f64" => {
                let casted: f64 = value.parse().map_err(|_| {
                    syn::Error::new_spanned(
                        self.rust_type.to_token_stream(),
                        format!("Cannot cast value to external type: {:?}", self),
                    )
                })?;
                quote! { #casted }
            }
            "u128" => {
                let casted: u128 = value.parse().map_err(|_| {
                    syn::Error::new_spanned(
                        self.rust_type.to_token_stream(),
                        format!("Cannot cast value to external type: {:?}", self),
                    )
                })?;
                quote! { #casted }
            }
            "i128" => {
                let casted: i128 = value.parse().map_err(|_| {
                    syn::Error::new_spanned(
                        self.rust_type.to_token_stream(),
                        format!("Cannot cast value to external type: {:?}", self),
                    )
                })?;
                quote! { #casted }
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    self.rust_type.to_token_stream(),
                    format!("Cannot cast value to external type: {:?}", self),
                ));
            }
        })
    }

    /// Returns an iterator over the generic idents without defaults.
    pub fn generics_without_defaults(&self) -> impl Iterator<Item = &Ident> {
        self.generics.iter().enumerate().filter_map(|(i, generic)| {
            if self.generic_defaults[i].is_none() { Some(generic) } else { None }
        })
    }

    /// Returns the formatted generics, with defaults in place of the generic
    /// where they exist.
    pub fn generics_with_defaults(&self) -> Option<TokenStream> {
        if self.generics.is_empty() {
            None
        } else {
            let generics_with_defaults =
                self.generics.iter().zip(self.generic_defaults.iter()).map(|(ident, default)| {
                    if let Some(default) = default {
                        quote::quote! { #default }
                    } else {
                        quote::quote! { #ident }
                    }
                });
            Some(quote::quote! { <#(#generics_with_defaults),*> })
        }
    }

    /// Sets a generic field to the provided `DataVariantRef`.
    pub fn set_generic_field(&self, field: &Ident, value: DataVariantRef) -> Option<Self> {
        if !self.generics.contains(field) {
            return None;
        }

        let mut new = self.clone();
        for (i, generic) in self.generics.iter().enumerate() {
            if generic == field {
                new.generic_defaults[i] = Some(value);
                return Some(new);
            }
        }

        unreachable!()
    }
}

impl ExternalDependencies for ExternalType {
    fn external_dependencies(&self) -> Vec<Arc<crate::structs::ExternalCrate>> {
        let mut dependencies = Vec::new();
        for default in &self.generic_defaults {
            dependencies.extend(default.external_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}

impl InternalDependencies for ExternalType {
    fn internal_dependencies(&self) -> Vec<&crate::structs::InternalCrate> {
        let mut dependencies = Vec::new();
        for default in &self.generic_defaults {
            dependencies.extend(default.internal_dependencies());
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}
