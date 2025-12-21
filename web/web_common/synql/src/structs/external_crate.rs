//! Submodule providing a struct defining the crate required by some type found
//! in the postgres database schema.

use std::{fmt::Debug, hash::Hash};

pub use builder::ExternalCrateBuilder;
use quote::ToTokens;
use syn::Type;

use crate::structs::{
    ExternalFunction, ExternalFunctionRef, ExternalType, ExternalTypeRef,
    external_crate::builder::ExternalCrateBuilderError,
};
mod builder;
mod chrono_crate;
mod core_crate;
mod diesel_builders;
mod diesel_crate;
mod pgrx_validation;
mod postgis_diesel_crate;
mod rosetta_timestamp;
mod rosetta_uuid_crate;
mod serde;
mod std_crate;
mod validation_errors;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// Struct defining the crate required by some type found in the postgres
/// database schema.
pub struct ExternalCrate {
    /// The name of the crate.
    name: String,
    /// List of postgres types and their corresponding diesel and rust types
    /// defined within the crate.
    types: Vec<ExternalType>,
    /// Methods defined within the crate.
    functions: Vec<ExternalFunction>,
    /// Whether the crate is a dependency.
    version: Option<String>,
    /// Git to the crate, if it is a local dependency.
    git: Option<(String, String)>,
    /// Feature flags required by the crate.
    features: Vec<String>,
}

impl PartialOrd for ExternalCrate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl Ord for ExternalCrate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

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
    pub fn external_type(&self, ident: &Type) -> Option<ExternalTypeRef<'_>> {
        self.types
            .iter()
            .find(|t| {
                t.rust_type().to_token_stream().to_string() == ident.to_token_stream().to_string()
            })
            .map(|t| ExternalTypeRef::new(self, t))
    }

    /// Returns the external function ref with the provided name, if any.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the external
    ///   function.
    #[must_use]
    pub fn external_function_ref(&self, name: &str) -> Option<ExternalFunctionRef<'_>> {
        self.functions.iter().find(|f| f.name() == name).map(|f| ExternalFunctionRef::new(self, f))
    }

    /// Returns the external type compatible with the provided postgres name, if
    /// any.
    ///
    /// # Arguments
    /// * `postgres_type` - The postgres type to find a compatible type for.
    #[must_use]
    pub fn external_postgres_type(&self, postgres_type: &str) -> Option<ExternalTypeRef<'_>> {
        self.types
            .iter()
            .find(|t| t.is_compatible_with(postgres_type))
            .map(|t| ExternalTypeRef::new(self, t))
    }
}
