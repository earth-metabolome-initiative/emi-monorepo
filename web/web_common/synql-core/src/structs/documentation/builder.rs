//! Submodule defining a builder for the `Documentation` struct.

use std::{error::Error, fmt::Display, sync::Arc};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::structs::{Documentation, ExternalCrate, InternalCrate};

#[derive(Default)]
/// Builder for the `Documentation` struct.
pub struct DocumentationBuilder {
    /// The documentation string.
    documentation: Option<String>,
    /// The external crate dependencies required by this documentation.
    external_dependencies: Vec<Arc<ExternalCrate>>,
    /// The internal crate dependencies required by this documentation.
    internal_dependencies: Vec<Arc<InternalCrate>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `Documentation` struct.
pub enum DocumentationAttribute {
    /// The documentation string.
    Documentation,
    /// The external crate dependencies required by this documentation.
    ExternalDependencies,
    /// The internal crate dependencies required by this documentation.
    InternalDependencies,
}

impl Display for DocumentationAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DocumentationAttribute::Documentation => write!(f, "documentation"),
            DocumentationAttribute::ExternalDependencies => write!(f, "external dependencies"),
            DocumentationAttribute::InternalDependencies => write!(f, "internal dependencies"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of a
/// `Documentation`.
pub enum DocumentationBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<DocumentationAttribute>),
    /// The documentation string is invalid (empty or whitespace only).
    InvalidDocumentation,
    /// An external crate dependency was specified but does not appear in the
    /// documentation string.
    UnexpectedExternalCrateDependency(String, String),
    /// An internal crate dependency was specified but does not appear in the
    /// documentation string.
    UnexpectedInternalCrateDependency(String, String),
    /// A duplicated external crate dependency was added.
    DuplicatedExternalCrateDependency,
    /// A duplicated internal crate dependency was added.
    DuplicatedInternalCrateDependency,
}

impl Display for DocumentationBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DocumentationBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            DocumentationBuilderError::InvalidDocumentation => {
                write!(f, "Invalid documentation string (empty or whitespace only)")
            }
            DocumentationBuilderError::UnexpectedExternalCrateDependency(
                crate_name,
                documentation,
            ) => {
                write!(
                    f,
                    "External crate dependency '{crate_name}' does not appear in the documentation string: {documentation}",
                )
            }
            DocumentationBuilderError::UnexpectedInternalCrateDependency(
                crate_name,
                documentation,
            ) => {
                write!(
                    f,
                    "Internal crate dependency '{crate_name}' does not appear in the documentation string: {documentation}",
                )
            }
            DocumentationBuilderError::DuplicatedExternalCrateDependency => {
                write!(f, "A duplicated external crate dependency was added")
            }
            DocumentationBuilderError::DuplicatedInternalCrateDependency => {
                write!(f, "A duplicated internal crate dependency was added")
            }
        }
    }
}

impl Error for DocumentationBuilderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DocumentationBuilderError::Builder(e) => Some(e),
            _ => None,
        }
    }
}

impl From<BuilderError<DocumentationAttribute>> for DocumentationBuilderError {
    fn from(e: BuilderError<DocumentationAttribute>) -> Self {
        DocumentationBuilderError::Builder(e)
    }
}

impl DocumentationBuilder {
    /// Sets the documentation string.
    ///
    /// # Arguments
    /// * `documentation` - The documentation string.
    pub fn documentation<S: ToString>(
        mut self,
        documentation: S,
    ) -> Result<Self, DocumentationBuilderError> {
        let documentation = documentation.to_string();
        if documentation.trim().is_empty() {
            return Err(DocumentationBuilderError::InvalidDocumentation);
        }
        self.documentation = Some(documentation);
        Ok(self)
    }

    /// Adds an external crate dependency to the documentation.
    ///
    /// # Arguments
    /// * `external_crate` - The external crate dependency.
    pub fn external_dependency(
        mut self,
        external_crate: Arc<ExternalCrate>,
    ) -> Result<Self, DocumentationBuilderError> {
        if self.external_dependencies.iter().any(|c| c.name() == external_crate.name()) {
            return Err(DocumentationBuilderError::DuplicatedExternalCrateDependency);
        }
        self.external_dependencies.push(external_crate);
        Ok(self)
    }

    /// Adds multiple external crate dependencies to the documentation.
    ///
    /// # Arguments
    /// * `external_crates` - The external crate dependencies.
    pub fn external_dependencies<I>(
        mut self,
        external_crates: I,
    ) -> Result<Self, DocumentationBuilderError>
    where
        I: IntoIterator<Item = Arc<ExternalCrate>>,
    {
        for external_crate in external_crates {
            self = self.external_dependency(external_crate)?;
        }
        Ok(self)
    }

    /// Adds an internal crate dependency to the documentation.
    ///
    /// # Arguments
    /// * `internal_crate` - The internal crate dependency.
    pub fn internal_dependency(
        mut self,
        internal_crate: Arc<InternalCrate>,
    ) -> Result<Self, DocumentationBuilderError> {
        if self.internal_dependencies.iter().any(|c| c.name() == internal_crate.name()) {
            return Err(DocumentationBuilderError::DuplicatedInternalCrateDependency);
        }
        self.internal_dependencies.push(internal_crate);
        Ok(self)
    }

    /// Adds multiple internal crate dependencies to the documentation.
    ///
    /// # Arguments
    /// * `internal_crates` - The internal crate dependencies.
    pub fn internal_dependencies<I>(
        mut self,
        internal_crates: I,
    ) -> Result<Self, DocumentationBuilderError>
    where
        I: IntoIterator<Item = Arc<InternalCrate>>,
    {
        for internal_crate in internal_crates {
            self = self.internal_dependency(internal_crate)?;
        }
        Ok(self)
    }
}

impl Attributed for DocumentationBuilder {
    type Attribute = DocumentationAttribute;
}

impl IsCompleteBuilder for DocumentationBuilder {
    fn is_complete(&self) -> bool {
        self.documentation.is_some()
    }
}

impl Builder for DocumentationBuilder {
    type Error = DocumentationBuilderError;
    type Object = Documentation;

    fn build(self) -> Result<Self::Object, Self::Error> {
        let documentation = self
            .documentation
            .ok_or(BuilderError::IncompleteBuild(DocumentationAttribute::Documentation))?;

        // Validate that external crate dependencies appear in the documentation string
        for external_crate in &self.external_dependencies {
            if !documentation.contains(external_crate.name()) {
                return Err(DocumentationBuilderError::UnexpectedExternalCrateDependency(
                    external_crate.name().to_string(),
                    documentation.clone(),
                ));
            }
        }

        // Validate that internal crate dependencies appear in the documentation string
        for internal_crate in &self.internal_dependencies {
            if !documentation.contains(internal_crate.name()) {
                return Err(DocumentationBuilderError::UnexpectedInternalCrateDependency(
                    internal_crate.name().to_string(),
                    documentation.clone(),
                ));
            }
        }

        Ok(Documentation {
            documentation,
            external_dependencies: self.external_dependencies,
            internal_dependencies: self.internal_dependencies,
        })
    }
}
