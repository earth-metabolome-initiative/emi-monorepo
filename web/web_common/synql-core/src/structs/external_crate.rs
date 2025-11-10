//! Submodule providing a struct defining the crate required by some type found
//! in the postgres database schema.

use std::{fmt::Debug, hash::Hash, sync::Arc};

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Ident, Type};

use crate::{
    structs::{
        DataVariantRef, ExternalFunctionRef, ExternalMacro, ExternalTrait, ExternalType, Method,
        Trait, external_crate::builder::ExternalCrateBuilder, external_trait::TraitVariantRef,
    },
    traits::{ExternalDependencies, InternalDependencies},
};

mod builder;
mod chrono_crate;
mod core_crate;
mod diesel_crate;
mod diesel_queries_crate;
mod helpers;
mod pgrx_validation;
mod postgis_diesel_crate;
mod rosetta_uuid_crate;
mod serde_crate;
mod std_crate;
mod validation_errors_crate;

#[derive(Clone)]
/// Struct defining the crate required by some type found in the postgres
/// database schema.
pub struct ExternalCrate {
    /// The name of the crate.
    name: String,
    /// List of postgres types and their corresponding diesel and rust types
    /// defined within the crate.
    types: Vec<Arc<ExternalType>>,
    /// List of the macros defined within the crate.
    macros: Vec<ExternalMacro>,
    /// List of the traits defined within the crate.
    traits: Vec<ExternalTrait>,
    /// Methods defined within the crate.
    functions: Vec<(Arc<Method>, Arc<syn::Path>)>,
    /// Whether the crate is a dependency.
    version: Option<String>,
    /// Git to the crate, if it is a local dependency.
    git: Option<(String, String)>,
    /// Feature flags required by the crate.
    features: Vec<String>,
}

impl Debug for ExternalCrate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExternalCrate")
            .field("name", &self.name)
            .field("types", &self.types)
            .field("macros", &self.macros)
            .field("traits", &self.traits)
            .field("functions", &self.functions.iter().map(|(m, _)| m).collect::<Vec<_>>())
            .field("version", &self.version)
            .field("git", &self.git)
            .field("features", &self.features)
            .finish()
    }
}

impl PartialEq for ExternalCrate {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for ExternalCrate {}

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

impl Hash for ExternalCrate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

unsafe impl Send for ExternalCrate {}
unsafe impl Sync for ExternalCrate {}

impl ExternalCrate {
    /// Inizializes a new `ExternalCrateBuilder`.
    pub fn new() -> ExternalCrateBuilder {
        ExternalCrateBuilder::default()
    }

    /// Returns a reference to the name of the crate.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the crate is a dependency.
    pub fn is_dependency(&self) -> bool {
        self.version.is_some() || self.git.is_some()
    }

    /// Returns the version of the crate if it is a dependency.
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Returns the git repository and branch of the crate if it is a
    /// dependency.
    pub fn git(&self) -> Option<(&str, &str)> {
        self.git.as_ref().map(|(repo, branch)| (repo.as_str(), branch.as_str()))
    }

    /// Returns the feature flags required by the crate.
    pub fn features(&self) -> &[String] {
        &self.features
    }

    /// Returns the external type with the provided name, if any.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the external type.
    pub fn external_type(self: &Arc<Self>, ident: &Type) -> Option<ExternalTypeRef> {
        self.types
            .iter()
            .find(|t| {
                t.rust_type().to_token_stream().to_string() == ident.to_token_stream().to_string()
            })
            .map(|t| ExternalTypeRef { crate_ref: self.clone(), type_ref: t.clone() })
    }

    /// Returns the external macro with the provided name, if any.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the external macro.
    pub fn external_macro(self: &Arc<Self>, name: &str) -> Option<ExternalMacroRef> {
        self.macros
            .iter()
            .find(|m| m.name() == name)
            .map(|m| ExternalMacroRef { crate_ref: self.clone(), macro_ref: Arc::new(m.clone()) })
    }

    /// Returns the external trait with the provided name, if any.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the external trait.
    pub fn external_trait(&self, name: &str) -> Option<&ExternalTrait> {
        self.traits.iter().find(|t| t.name() == name)
    }

    /// Returns an iterator over all external trait refs defined within the
    /// crate.
    pub fn external_trait_refs(self: &Arc<Self>) -> impl Iterator<Item = ExternalTraitRef> {
        let crate_ref = self.clone();
        self.traits.iter().map(move |t| {
            ExternalTraitRef { crate_ref: crate_ref.clone(), trait_ref: Arc::new(t.clone()) }
        })
    }

    /// Returns the external trait ref with the provided name, if any.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the external trait.
    pub fn external_trait_ref(self: &Arc<Self>, name: &str) -> Option<ExternalTraitRef> {
        self.external_trait(name)
            .map(|t| ExternalTraitRef { crate_ref: self.clone(), trait_ref: Arc::new(t.clone()) })
    }

    /// Returns the external function ref with the provided name, if any.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the external
    ///   function.
    pub fn external_function_ref(&self, name: &str) -> Option<ExternalFunctionRef> {
        self.functions.iter().find(|(m, _)| m.name() == name).map(|(m, path)| {
            ExternalFunctionRef::new(m.clone(), path.clone(), Arc::new(self.clone()))
        })
    }

    /// Returns the external type compatible with the provided postgres name, if
    /// any.
    ///
    /// # Arguments
    /// * `postgres_type` - The postgres type to find a compatible type for.
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a reference to an external crate and one of its types.
pub struct ExternalTypeRef {
    crate_ref: Arc<ExternalCrate>,
    type_ref: Arc<ExternalType>,
}

impl ExternalTypeRef {
    /// Returns a reference to the name of the crate.
    pub fn crate_name(&self) -> &str {
        self.crate_ref.name()
    }

    /// Returns a reference to the diesel type.
    pub fn diesel_type(&self) -> Option<&syn::Type> {
        self.type_ref.diesel_type()
    }

    /// Returns a reference to the rust type.
    pub fn rust_type(&self) -> &syn::Type {
        self.type_ref.rust_type()
    }

    /// Returns a reference to the external crate.
    pub fn external_crate(&self) -> &ExternalCrate {
        &self.crate_ref
    }

    /// Returns whether the crate is a dependency.
    pub fn is_dependency(&self) -> bool {
        self.crate_ref.is_dependency()
    }

    /// Returns the version of the crate if it is a dependency.
    pub fn version(&self) -> Option<&str> {
        self.crate_ref.version()
    }

    /// Returns the git repository and branch of the crate if it is a
    /// dependency.
    pub fn git(&self) -> Option<(&str, &str)> {
        self.crate_ref.git()
    }

    /// Returns whether the type supports the given trait.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait variant to check support for.
    pub fn supports_trait(&self, trait_ref: &TraitVariantRef) -> bool {
        self.type_ref.supports(trait_ref)
    }

    /// Returns whether the type supports the `Copy` trait in Rust.
    pub fn supports_copy(&self) -> bool {
        self.type_ref.supports(&Trait::Copy.into())
    }

    /// Returns whether the type is a `Unit` type.
    pub fn is_unit(&self) -> bool {
        self.type_ref.is_unit()
    }

    /// Casts a value to the external type.
    pub(crate) fn cast(&self, value: &str) -> Result<proc_macro2::TokenStream, syn::Error> {
        self.type_ref.cast(value)
    }

    /// Returns an iterator over the generic idents without defaults.
    pub fn generics_without_defaults(&self) -> impl Iterator<Item = &Ident> {
        self.type_ref.generics_without_defaults()
    }

    /// Sets a generic field to the provided `DataVariantRef`.
    pub fn set_generic_field(&self, field: &Ident, value: DataVariantRef) -> Option<Self> {
        Some(Self {
            type_ref: Arc::new(self.type_ref.set_generic_field(field, value)?),
            crate_ref: self.crate_ref.clone(),
        })
    }

    /// Formats the variant including the generics, if any, with defaults.
    pub fn format_with_generics(&self) -> TokenStream {
        let generics = self.type_ref.generics_with_defaults();
        quote::quote! { #self #generics }
    }
}

impl ExternalDependencies for ExternalTypeRef {
    fn external_dependencies(&self) -> Vec<Arc<ExternalCrate>> {
        let mut dependencies = vec![self.crate_ref.clone()];
        dependencies.extend(self.type_ref.external_dependencies());
        dependencies.sort_unstable();
        dependencies.dedup();
        dependencies
    }
}

impl InternalDependencies for ExternalTypeRef {
    fn internal_dependencies(&self) -> Vec<&crate::structs::InternalCrate> {
        self.type_ref.internal_dependencies()
    }
}

impl ToTokens for ExternalTypeRef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.type_ref.rust_type().to_tokens(tokens);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a reference to an external crate and one of its macros.
pub struct ExternalMacroRef {
    crate_ref: Arc<ExternalCrate>,
    macro_ref: Arc<ExternalMacro>,
}

impl ExternalMacroRef {
    /// Returns a reference to the name of the crate.
    pub fn crate_name(&self) -> &str {
        self.crate_ref.name()
    }

    /// Returns a reference to the name of the macro.
    pub fn name(&self) -> &str {
        self.macro_ref.name()
    }

    /// Returns a reference to the external crate.
    pub fn external_crate(&self) -> &ExternalCrate {
        &self.crate_ref
    }

    /// Returns a reference to the external macro.
    pub fn external_macro(&self) -> &ExternalMacro {
        &self.macro_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a reference to an external crate and one of its traits.
pub struct ExternalTraitRef {
    crate_ref: Arc<ExternalCrate>,
    trait_ref: Arc<ExternalTrait>,
}

impl From<Trait> for ExternalTraitRef {
    fn from(trait_def: Trait) -> Self {
        let core = ExternalCrate::core();
        Self {
            trait_ref: Arc::new(
                core.external_trait(trait_def.as_ref())
                    .expect(&format!("Trait `{trait_def}` should exist"))
                    .clone(),
            ),
            crate_ref: core,
        }
    }
}

impl ExternalDependencies for ExternalTraitRef {
    fn external_dependencies(&self) -> Vec<Arc<ExternalCrate>> {
        vec![self.crate_ref.clone()]
    }
}

impl ToTokens for ExternalTraitRef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.trait_ref.path().to_token_stream());
    }
}

impl ExternalTraitRef {
    /// Returns a reference to the name of the crate.
    pub fn crate_name(&self) -> &str {
        self.crate_ref.name()
    }

    /// Returns a reference to the name of the trait.
    pub fn name(&self) -> &str {
        self.trait_ref.name()
    }

    /// Returns a reference to the external crate.
    pub fn external_crate(&self) -> &ExternalCrate {
        &self.crate_ref
    }

    /// Returns a reference to the external trait.
    pub fn external_trait(&self) -> &ExternalTrait {
        &self.trait_ref
    }

    /// Returns whether the trait is implemented for typeless enums.
    pub fn implemented_for_typeless_enum(&self) -> bool {
        self.trait_ref.implemented_for_typeless_enum()
    }
}
