//! Submodule defining a builder for the `Workspace` struct.

use std::{error::Error, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::structs::{ExternalCrate, Workspace};

#[derive(Default)]
/// Builder for the `Workspace` struct.
pub struct WorkspaceBuilder {
    /// External crates made available within the workspace.
    external_crates: Vec<ExternalCrate>,
    /// Name of the workspace.
    name: Option<String>,
    /// Version of the workspace.
    version: Option<(u8, u8, u8)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `Workspace` struct.
pub enum WorkspaceAttribute {
    /// External crates made available within the workspace.
    ExternalCrates,
    /// Name of the workspace.
    Name,
    /// Version of the workspace.
    Version,
}

impl Display for WorkspaceAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkspaceAttribute::ExternalCrates => write!(f, "external_crates"),
            WorkspaceAttribute::Name => write!(f, "name"),
            WorkspaceAttribute::Version => write!(f, "version"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of a
/// `Workspace`.
pub enum WorkspaceBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<WorkspaceAttribute>),
    /// The name of the workspace is invalid.
    InvalidName,
    /// A crate with the same name has already been added to the
    /// workspace.
    DuplicatedCrateName,
}

impl Display for WorkspaceBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkspaceBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            WorkspaceBuilderError::InvalidName => write!(f, "Invalid workspace name"),
            WorkspaceBuilderError::DuplicatedCrateName => {
                write!(
                    f,
                    "A crate with the same name has already been added to the workspace"
                )
            }
        }
    }
}

impl Error for WorkspaceBuilderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            WorkspaceBuilderError::Builder(e) => Some(e),
            _ => None,
        }
    }
}

impl WorkspaceBuilder {
    /// Sets the name of the workspace.
    ///
    /// # Arguments
    /// * `name` - The name of the workspace.
    pub fn name<S: ToString>(mut self, name: S) -> Result<Self, WorkspaceBuilderError> {
        let name = name.to_string();
        if name.trim().is_empty() || name.contains(' ') {
            return Err(WorkspaceBuilderError::InvalidName);
        }
        self.name = Some(name);
        Ok(self)
    }

    /// Sets the version of the workspace.
    ///
    /// # Arguments
    /// * `major` - The major version number.
    /// * `minor` - The minor version number.
    /// * `patch` - The patch version number.
    pub fn version(mut self, major: u8, minor: u8, patch: u8) -> Self {
        self.version = Some((major, minor, patch));
        self
    }

    /// Adds an external crate to the workspace.
    ///
    /// # Arguments
    /// * `external_crate` - The external crate to add.
    pub fn add_external_crate(
        mut self,
        external_crate: ExternalCrate,
    ) -> Result<Self, WorkspaceBuilderError> {
        if self
            .external_crates
            .iter()
            .any(|c| c.name() == external_crate.name())
        {
            return Err(WorkspaceBuilderError::DuplicatedCrateName);
        }
        self.external_crates.push(external_crate);
        Ok(self)
    }

	/// Adds the `std` external crate to the workspace.
	pub fn std(self) -> Result<Self, WorkspaceBuilderError> {
		self.add_external_crate(ExternalCrate::std())
	}

	// Adds the core external crate to the workspace.
	pub fn core(self) -> Result<Self, WorkspaceBuilderError> {
		self.add_external_crate(ExternalCrate::core())
	}

    /// Adds multiple external crates to the workspace.
    ///
    /// # Arguments
    /// * `external_crates` - The external crates to add.
    pub fn add_external_crates<I>(
        mut self,
        external_crates: I,
    ) -> Result<Self, WorkspaceBuilderError>
    where
        I: IntoIterator<Item = ExternalCrate>,
    {
        for external_crate in external_crates {
            self = self.add_external_crate(external_crate)?;
        }
        Ok(self)
    }
}

impl Attributed for WorkspaceBuilder {
    type Attribute = WorkspaceAttribute;
}

impl IsCompleteBuilder for WorkspaceBuilder {
    fn is_complete(&self) -> bool {
        self.name.is_some() && self.version.is_some()
    }
}

impl Builder for WorkspaceBuilder {
    type Error = BuilderError<WorkspaceAttribute>;
    type Object = Workspace;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Workspace {
            external_crates: self.external_crates,
            name: self
                .name
                .ok_or(BuilderError::IncompleteBuild(WorkspaceAttribute::Name))?,
            version: self
                .version
                .ok_or(BuilderError::IncompleteBuild(WorkspaceAttribute::Version))?,
            internal_crates: Vec::new(), // Internal crates are added later in the process.
        })
    }
}
