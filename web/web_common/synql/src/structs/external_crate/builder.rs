//! Submodule providing a builder for the `ExternalCrate` struct.

use std::{error::Error, fmt::Display, sync::Arc};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::structs::{ExternalCrate, ExternalMacro, TraitDef, ExternalType, Method};

#[derive(Default)]
/// Builder for the `ExternalCrate` struct.
pub struct ExternalCrateBuilder {
    /// The name of the crate.
    name: Option<String>,
    /// The types provided by the crate.
    types: Vec<Arc<ExternalType>>,
    /// List of the macros defined within the crate.
    macros: Vec<ExternalMacro>,
    /// List of the traits defined within the crate.
    traits: Vec<TraitDef>,
    /// The version of the crate if it is a dependency.
    version: Option<String>,
    /// Git to the crate, if it is a GitHub dependency.
    git: Option<(String, String)>,
    /// The feature flags required by the crate.
    features: Vec<String>,
    /// The functions provided by the crate.
    functions: Vec<(Arc<Method>, syn::Path)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `ExternalCrate` struct.
pub enum ExternalCrateAttribute {
    /// The name of the crate.
    Name,
    /// The types provided by the crate.
    Types,
    /// List of the macros defined within the crate.
    Macros,
    /// List of the traits defined within the crate.
    Traits,
    /// The version of the crate if it is a dependency.
    Version,
    /// The feature flags required by the crate.
    Features,
    /// The functions provided by the crate.
    Functions,
}

impl Display for ExternalCrateAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExternalCrateAttribute::Name => write!(f, "name"),
            ExternalCrateAttribute::Types => write!(f, "types"),
            ExternalCrateAttribute::Macros => write!(f, "macros"),
            ExternalCrateAttribute::Traits => write!(f, "traits"),
            ExternalCrateAttribute::Version => write!(f, "version"),
            ExternalCrateAttribute::Features => write!(f, "features"),
            ExternalCrateAttribute::Functions => write!(f, "functions"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of a
/// `ExternalCrate`.
pub enum ExternalCrateBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<ExternalCrateAttribute>),
    /// The name of the crate is invalid.
    InvalidName,
    /// A type handling the same postgres type has already been added to the
    /// crate.
    DuplicatedPostgresType,
    /// A macro with the same name has already been added to the crate.
    DuplicatedMacro,
    /// A trait with the same name has already been added to the crate.
    DuplicatedTrait,
}

impl Display for ExternalCrateBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExternalCrateBuilderError::Builder(e) => write!(f, "Builder error: {e}"),
            ExternalCrateBuilderError::InvalidName => write!(f, "Invalid crate name"),
            ExternalCrateBuilderError::DuplicatedPostgresType => {
                write!(
                    f,
                    "A type handling the same postgres type has already been added to the crate"
                )
            }
            ExternalCrateBuilderError::DuplicatedMacro => {
                write!(f, "A macro with the same name has already been added to the crate")
            }
            ExternalCrateBuilderError::DuplicatedTrait => {
                write!(f, "A trait with the same name has already been added to the crate")
            }
        }
    }
}

impl Error for ExternalCrateBuilderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ExternalCrateBuilderError::Builder(e) => Some(e),
            _ => None,
        }
    }
}

impl ExternalCrateBuilder {
    /// Sets the name of the crate.
    ///
    /// # Arguments
    /// * `name` - The name of the crate.
    pub fn name<S: ToString + ?Sized>(
        mut self,
        name: &S,
    ) -> Result<Self, ExternalCrateBuilderError> {
        let name = name.to_string();
        if name.trim().is_empty() || name.contains(' ') {
            return Err(ExternalCrateBuilderError::InvalidName);
        }
        self.name = Some(name);
        Ok(self)
    }

    /// Adds a type provided by the crate.
    ///
    /// # Arguments
    /// * `required_type` - The type provided by the crate.
    pub fn add_type(
        mut self,
        required_type: Arc<ExternalType>,
    ) -> Result<Self, ExternalCrateBuilderError> {
        for postgres_type in required_type.postgres_types() {
            if self.types.iter().any(|t| t.is_compatible_with(postgres_type)) {
                return Err(ExternalCrateBuilderError::DuplicatedPostgresType);
            }
        }
        self.types.push(required_type);
        Ok(self)
    }

    /// Adds the provided types to the crate.
    ///
    /// # Arguments
    /// * `required_types` - The types provided by the crate.
    pub fn add_types<I>(mut self, required_types: I) -> Result<Self, ExternalCrateBuilderError>
    where
        I: IntoIterator<Item = Arc<ExternalType>>,
    {
        for required_type in required_types {
            self = self.add_type(required_type)?;
        }
        Ok(self)
    }

    /// Sets whether the crate is a dependency.
    pub fn version<S: ToString + ?Sized>(mut self, version: &S) -> Self {
        self.version = Some(version.to_string());
        self
    }

    /// Sets the git to the crate, if it is a local dependency.
    pub fn git<S: ToString + ?Sized>(mut self, repository: &S, branch: &S) -> Self {
        self.git = Some((repository.to_string(), branch.to_string()));
        self
    }

    /// Adds a macro defined within the crate.
    ///
    /// # Arguments
    /// * `external_macro` - The macro to add.
    ///
    /// # Errors
    ///
    /// Returns an error if a macro with the same name has already been added.
    pub fn add_macro(
        mut self,
        external_macro: ExternalMacro,
    ) -> Result<Self, ExternalCrateBuilderError> {
        if self.macros.iter().any(|m| m.name() == external_macro.name()) {
            return Err(ExternalCrateBuilderError::DuplicatedMacro);
        }
        self.macros.push(external_macro);
        Ok(self)
    }

    /// Adds several macros defined within the crate.
    ///
    /// # Arguments
    /// * `external_macros` - The macros to add.
    ///
    /// # Errors
    ///
    /// Returns an error if any macro with the same name has already been added.
    pub fn add_macros<I>(mut self, external_macros: I) -> Result<Self, ExternalCrateBuilderError>
    where
        I: IntoIterator<Item = ExternalMacro>,
    {
        for external_macro in external_macros {
            self = self.add_macro(external_macro)?;
        }
        Ok(self)
    }

    /// Adds a trait defined within the crate.
    ///
    /// # Arguments
    /// * `external_trait` - The trait to add.
    ///
    /// # Errors
    ///
    /// Returns an error if a trait with the same name has already been added.
    pub fn add_trait(
        mut self,
        external_trait: TraitDef,
    ) -> Result<Self, ExternalCrateBuilderError> {
        if self.traits.iter().any(|t| t.name() == external_trait.name()) {
            return Err(ExternalCrateBuilderError::DuplicatedTrait);
        }
        self.traits.push(external_trait);
        Ok(self)
    }

    /// Adds several traits defined within the crate.
    ///
    /// # Arguments
    /// * `external_traits` - The traits to add.
    ///
    /// # Errors
    ///
    /// Returns an error if any trait with the same name has already been added.
    pub fn add_traits<I>(mut self, external_traits: I) -> Result<Self, ExternalCrateBuilderError>
    where
        I: IntoIterator<Item = TraitDef>,
    {
        for external_trait in external_traits {
            self = self.add_trait(external_trait)?;
        }
        Ok(self)
    }

    /// Adds a feature to the crate.
    ///
    /// # Arguments
    /// * `feature` - The feature to add.
    pub fn feature<S: ToString + ?Sized>(mut self, feature: &S) -> Self {
        let feature = feature.to_string();
        if !self.features.contains(&feature) {
            self.features.push(feature);
        }
        self
    }

    /// Adds several features required by the crate.
    ///
    /// # Arguments
    /// * `features` - The features to add.
    pub fn features<I, S>(mut self, features: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: ToString,
    {
        for feature in features {
            self = self.feature(&feature);
        }
        self
    }

    /// Adds a function provided by the crate.
    ///
    /// # Arguments
    ///
    /// * `method` - The method signature of the function.
    /// * `path` - The path to the function.
    pub fn add_function(mut self, method: Arc<Method>, path: syn::Path) -> Self {
        for function in &self.functions {
            if function.0 == method {
                return self;
            }
        }

        self.functions.push((method, path));
        self
    }

    /// Adds several functions provided by the crate.
    ///
    /// # Arguments
    /// * `functions` - The functions to add.
    pub fn add_functions<I>(mut self, functions: I) -> Self
    where
        I: IntoIterator<Item = (Arc<Method>, syn::Path)>,
    {
        for (method, path) in functions {
            self = self.add_function(method, path);
        }
        self
    }
}

impl Attributed for ExternalCrateBuilder {
    type Attribute = ExternalCrateAttribute;
}

impl IsCompleteBuilder for ExternalCrateBuilder {
    fn is_complete(&self) -> bool {
        self.name.is_some()
    }
}

impl Builder for ExternalCrateBuilder {
    type Error = BuilderError<ExternalCrateAttribute>;
    type Object = ExternalCrate;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(ExternalCrate {
            name: self.name.ok_or(BuilderError::IncompleteBuild(ExternalCrateAttribute::Name))?,
            types: self.types,
            macros: self.macros,
            traits: self.traits,
            version: self.version,
            features: self.features,
            functions: self.functions,
            git: self.git,
        })
    }
}
