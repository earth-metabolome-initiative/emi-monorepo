//! Submodule providing a builder for the `ExternalCrate` struct.

use std::fmt::Display;

use crate::structs::{ExternalCrate, ExternalFunction, ExternalType};

/// Builder for the `ExternalCrate` struct.
pub struct ExternalCrateBuilder {
    /// The name of the crate.
    name: String,
    /// The types provided by the crate.
    types: Vec<ExternalType>,
    /// The version of the crate if it is a dependency.
    version: Option<String>,
    /// Git to the crate, if it is a GitHub dependency.
    git: Option<(String, String)>,
    /// The feature flags required by the crate.
    features: Vec<String>,
    /// The functions provided by the crate.
    functions: Vec<ExternalFunction>,
}

impl ExternalCrateBuilder {
    /// Creates a new `ExternalCrateBuilder`.
    pub fn new(name: &str) -> Result<Self, ExternalCrateBuilderError> {
        if name.trim().is_empty() || name.contains(' ') {
            return Err(ExternalCrateBuilderError::InvalidName);
        }
        Ok(Self {
            name: name.to_string(),
            types: Vec::new(),
            version: None,
            features: Vec::new(),
            functions: Vec::new(),
            git: None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of a
/// `ExternalCrate`.
pub enum ExternalCrateBuilderError {
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

impl ExternalCrateBuilder {
    /// Adds a type provided by the crate.
    ///
    /// # Arguments
    /// * `required_type` - The type provided by the crate.
    pub fn add_type(
        mut self,
        required_type: ExternalType,
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
    pub fn types<I>(mut self, required_types: I) -> Result<Self, ExternalCrateBuilderError>
    where
        I: IntoIterator<Item = ExternalType>,
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
    pub fn function(mut self, function: ExternalFunction) -> Self {
        self.functions.push(function);
        self
    }

    /// Adds several functions provided by the crate.
    ///
    /// # Arguments
    /// * `functions` - The functions to add.
    pub fn functions<I>(mut self, functions: I) -> Self
    where
        I: IntoIterator<Item = ExternalFunction>,
    {
        for function in functions {
            self = self.function(function);
        }
        self
    }
}

impl From<ExternalCrateBuilder> for ExternalCrate {
    fn from(value: ExternalCrateBuilder) -> Self {
        ExternalCrate {
            name: value.name,
            types: value.types,
            version: value.version,
            git: value.git,
            features: value.features,
            functions: value.functions,
        }
    }
}
