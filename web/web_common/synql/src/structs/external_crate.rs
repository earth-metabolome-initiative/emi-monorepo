//! Submodule providing a struct defining the crate required by some type found
//! in the postgres database schema.

use std::{fmt::Debug, hash::Hash, sync::Arc};

pub use builder::ExternalCrateBuilder;
use quote::ToTokens;
use syn::{Path, Type};

use crate::structs::{ExternalType, external_crate::builder::ExternalCrateBuilderError, external_type::Trait};
mod builder;
mod chrono_crate;
mod core_crate;
mod diesel_crate;
mod helpers;
mod pgrx_validation;
mod postgis_diesel_crate;
mod rosetta_uuid_crate;
mod std_crate;

#[derive(Clone, Debug)]
/// Struct defining the crate required by some type found in the postgres
/// database schema.
pub struct ExternalCrate {
    /// The name of the crate.
    name: String,
    /// List of postgres types and their corresponding diesel and rust types
    /// defined within the crate.
    types: Vec<ExternalType>,
    /// Methods defined within the crate.
    functions: Vec<(String, syn::Path)>,
    /// Whether the crate is a dependency.
    version: Option<String>,
    /// Git to the crate, if it is a local dependency.
    git: Option<(String, String)>,
    /// Feature flags required by the crate.
    features: Vec<String>,
}

impl PartialEq for ExternalCrate {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for ExternalCrate {}

impl Hash for ExternalCrate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialOrd for ExternalCrate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ExternalCrate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

unsafe impl Send for ExternalCrate {}
unsafe impl Sync for ExternalCrate {}

impl ExternalCrate {
    /// Inizializes a new `ExternalCrateBuilder`.
    #[must_use]
    pub fn new(name: &str) -> Result<ExternalCrateBuilder, ExternalCrateBuilderError> {
        ExternalCrateBuilder::new(name)
    }

    /// Returns a reference to the name of the crate.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns true if the crate is a dependency.
    #[inline]
    #[must_use]
    pub fn is_dependency(&self) -> bool {
        self.version.is_some() || self.git.is_some()
    }

    /// Returns the version of the crate if it is a dependency.
    #[inline]
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Returns the git repository and branch of the crate if it is a
    /// dependency.
    #[inline]
    #[must_use]
    pub fn git(&self) -> Option<(&str, &str)> {
        self.git.as_ref().map(|(repo, branch)| (repo.as_str(), branch.as_str()))
    }

    /// Returns the feature flags required by the crate.
    #[inline]
    #[must_use]
    pub fn features(&self) -> &[String] {
        &self.features
    }

    /// Returns the external type with the provided name, if any.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the external type.
    #[must_use]
    pub fn external_type(self: &Arc<Self>, ident: &Type) -> Option<ExternalTypeRef> {
        self.types
            .iter()
            .find(|t| {
                t.rust_type().to_token_stream().to_string() == ident.to_token_stream().to_string()
            })
            .map(|t| ExternalTypeRef { crate_ref: self.clone(), type_ref: t.clone() })
    }

    /// Returns the external function ref with the provided name, if any.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the external
    ///   function.
    #[must_use]
    pub fn external_function_ref(&self, name: &str) -> Option<&Path> {
        self.functions.iter().find(|(m, _)| m.name() == name).map(|(_, path)| path)
    }

    /// Returns the external type compatible with the provided postgres name, if
    /// any.
    ///
    /// # Arguments
    /// * `postgres_type` - The postgres type to find a compatible type for.
    #[must_use]
    pub fn external_postgres_type(
        self: &Arc<Self>,
        postgres_type: &str,
    ) -> Option<ExternalTypeRef> {
        self.types
            .iter()
            .find(|t| t.is_compatible_with(postgres_type))
            .map(|t| ExternalTypeRef { crate_ref: self.clone(), type_ref: t.clone() })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Struct representing a reference to an external crate and one of its types.
pub struct ExternalTypeRef {
    crate_ref: Arc<ExternalCrate>,
    type_ref: Arc<ExternalType>,
}

impl ExternalTypeRef {
    /// Returns a reference to the name of the crate.
    #[inline]
    #[must_use]
    pub fn crate_name(&self) -> &str {
        self.crate_ref.name()
    }

    /// Returns a reference to the diesel type.
    #[inline]
    #[must_use]
    pub fn diesel_type(&self) -> Option<&syn::Type> {
        self.type_ref.diesel_type()
    }

    /// Returns a reference to the rust type.
    #[inline]
    #[must_use]
    pub fn rust_type(&self) -> &syn::Type {
        self.type_ref.rust_type()
    }

    /// Returns a reference to the external crate.
    #[inline]
    #[must_use]
    pub fn external_crate(&self) -> &ExternalCrate {
        &self.crate_ref
    }

    /// Returns true if the crate is a dependency.
    #[inline]
    #[must_use]
    pub fn is_dependency(&self) -> bool {
        self.crate_ref.is_dependency()
    }

    /// Returns the version of the crate if it is a dependency.
    #[inline]
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.crate_ref.version()
    }

    /// Returns the git repository and branch of the crate if it is a
    /// dependency.
    #[inline]
    #[must_use]
    pub fn git(&self) -> Option<(&str, &str)> {
        self.crate_ref.git()
    }

    /// Returns true if the type supports the provided trait.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait variant to check support for.
    #[must_use]
    pub fn supports_trait(&self, trait_ref: &Trait) -> bool {
        self.type_ref.supports(trait_ref)
    }

    /// Returns whether the type supports the `Copy` trait in Rust.
    #[must_use]
    pub fn supports_copy(&self) -> bool {
        self.type_ref.supports(&Trait::Copy.into())
    }

    /// Returns whether the type is a unit type.
    #[must_use]
    pub fn is_unit(&self) -> bool {
        self.type_ref.is_unit()
    }

    /// Casts a value to the external type.
    pub(crate) fn cast(&self, value: &str) -> Result<proc_macro2::TokenStream, syn::Error> {
        self.type_ref.cast(value)
    }

    /// Returns an iterator over the generic idents without defaults.
    pub fn generics_without_defaults(&self) -> impl Iterator<Item = &syn::GenericParam> + '_ {
        self.type_ref.generics_without_defaults()
    }
}

impl ToTokens for ExternalTypeRef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.type_ref.rust_type().to_tokens(tokens);
    }
}
