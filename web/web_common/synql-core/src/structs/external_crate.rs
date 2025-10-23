//! Submodule providing a struct defining the crate required by some type found
//! in the postgres database schema.

use quote::ToTokens;

use crate::structs::{
    ExternalMacro, ExternalTrait, ExternalType, Trait,
    external_crate::builder::ExternalCrateBuilder, external_trait::TraitVariantRef,
};

mod builder;
mod chrono_crate;
mod core_crate;
mod diesel_crate;
mod postgis_diesel_crate;
mod serde_crate;
mod std_crate;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct defining the crate required by some type found in the postgres
/// database schema.
pub struct ExternalCrate<'data> {
    /// The name of the crate.
    name: String,
    /// List of postgres types and their corresponding diesel and rust types
    /// defined within the crate.
    types: Vec<ExternalType<'data>>,
    /// List of the macros defined within the crate.
    macros: Vec<ExternalMacro>,
    /// List of the traits defined within the crate.
    traits: Vec<ExternalTrait>,
    /// Whether the crate is a dependency.
    version: Option<String>,
    /// Feature flags required by the crate.
    features: Vec<String>,
}

impl<'data> ExternalCrate<'data> {
    /// Inizializes a new `ExternalCrateBuilder`.
    pub fn new() -> ExternalCrateBuilder<'data> {
        ExternalCrateBuilder::default()
    }

    /// Returns a reference to the name of the crate.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the name of the crate.
    ///
    /// # Arguments
    ///
    /// * `postgres_type` - The postgres type to find a compatible type for.
    pub fn compatible_type(&self, postgres_type: &str) -> Option<ExternalTypeRef<'_>> {
        self.types
            .iter()
            .find(|ty| ty.is_compatible_with(postgres_type))
            .map(|ty| ExternalTypeRef { crate_ref: self, type_ref: ty })
    }

    /// Returns whether the crate is a dependency.
    pub fn is_dependency(&self) -> bool {
        self.version.is_some()
    }

    /// Returns the version of the crate if it is a dependency.
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Returns the feature flags required by the crate.
    pub fn features(&self) -> &[String] {
        &self.features
    }

    /// Returns the external macro with the provided name, if any.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the external macro.
    pub fn external_macro(&self, name: &str) -> Option<ExternalMacroRef<'_>> {
        self.macros
            .iter()
            .find(|m| m.name() == name)
            .map(|m| ExternalMacroRef { crate_ref: self, macro_ref: m })
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
    pub fn external_trait_refs(&self) -> impl Iterator<Item = ExternalTraitRef<'_>> {
        self.traits.iter().map(|t| ExternalTraitRef { crate_ref: self, trait_ref: t })
    }

    /// Returns the external trait ref with the provided name, if any.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the external trait.
    pub fn external_trait_ref(&self, name: &str) -> Option<ExternalTraitRef<'_>> {
        self.external_trait(name).map(|t| ExternalTraitRef { crate_ref: self, trait_ref: t })
    }

    /// Returns the external type compatible with the provided postgres name, if
    /// any.
    ///
    /// # Arguments
    /// * `postgres_type` - The postgres type to find a compatible type for.
    pub fn external_postgres_type(&self, postgres_type: &str) -> Option<ExternalTypeRef<'_>> {
        self.types
            .iter()
            .find(|t| t.is_compatible_with(postgres_type))
            .map(|t| ExternalTypeRef { crate_ref: self, type_ref: t })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a reference to an external crate and one of its types.
pub struct ExternalTypeRef<'data> {
    crate_ref: &'data ExternalCrate<'data>,
    type_ref: &'data ExternalType<'data>,
}

impl<'data> ExternalTypeRef<'data> {
    /// Returns a reference to the name of the crate.
    pub fn crate_name(&self) -> &'data str {
        self.crate_ref.name()
    }

    /// Returns a reference to the diesel type.
    pub fn diesel_type(&self) -> &'data syn::Type {
        self.type_ref.diesel_type()
    }

    /// Returns a reference to the rust type.
    pub fn rust_type(&self) -> &'data syn::Type {
        self.type_ref.rust_type()
    }

    /// Returns a reference to the external crate.
    pub fn external_crate(&self) -> &'data ExternalCrate<'data> {
        self.crate_ref
    }

    /// Returns whether the crate is a dependency.
    pub fn is_dependency(&self) -> bool {
        self.crate_ref.is_dependency()
    }

    /// Returns the version of the crate if it is a dependency.
    pub fn version(&self) -> Option<&'data str> {
        self.crate_ref.version()
    }

    /// Returns whether the type supports the given trait.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait variant to check support for.
    pub fn supports_trait(&self, trait_ref: &TraitVariantRef<'data>) -> bool {
        self.type_ref.supports(trait_ref)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a reference to an external crate and one of its macros.
pub struct ExternalMacroRef<'data> {
    crate_ref: &'data ExternalCrate<'data>,
    macro_ref: &'data ExternalMacro,
}

impl<'data> ExternalMacroRef<'data> {
    /// Returns a reference to the name of the crate.
    pub fn crate_name(&self) -> &'data str {
        self.crate_ref.name()
    }

    /// Returns a reference to the name of the macro.
    pub fn name(&self) -> &'data str {
        self.macro_ref.name()
    }

    /// Returns a reference to the external crate.
    pub fn external_crate(&self) -> &'data ExternalCrate<'data> {
        self.crate_ref
    }

    /// Returns a reference to the external macro.
    pub fn external_macro(&self) -> &'data ExternalMacro {
        self.macro_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a reference to an external crate and one of its traits.
pub struct ExternalTraitRef<'data> {
    crate_ref: &'data ExternalCrate<'data>,
    trait_ref: &'data ExternalTrait,
}

impl From<Trait> for ExternalTraitRef<'static> {
    fn from(trait_def: Trait) -> Self {
        let core = ExternalCrate::core();
        Self {
            crate_ref: core,
            trait_ref: core
                .external_trait(trait_def.as_ref())
                .expect(&format!("Trait `{trait_def}` should exist")),
        }
    }
}

impl ToTokens for ExternalTraitRef<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.trait_ref.path().to_token_stream());
    }
}

impl<'data> ExternalTraitRef<'data> {
    /// Returns a reference to the name of the crate.
    pub fn crate_name(&self) -> &'data str {
        self.crate_ref.name()
    }

    /// Returns a reference to the name of the trait.
    pub fn name(&self) -> &'data str {
        self.trait_ref.name()
    }

    /// Returns a reference to the external crate.
    pub fn external_crate(&self) -> &'data ExternalCrate<'data> {
        self.crate_ref
    }

    /// Returns a reference to the external trait.
    pub fn external_trait(&self) -> &'data ExternalTrait {
        self.trait_ref
    }
}
