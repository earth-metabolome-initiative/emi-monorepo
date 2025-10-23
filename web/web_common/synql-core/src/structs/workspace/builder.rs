//! Submodule defining a builder for the `Workspace` struct.

use std::{error::Error, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::structs::{ExternalCrate, Workspace};

/// Builder for the `Workspace` struct.
pub struct WorkspaceBuilder<'data> {
    /// External crates made available within the workspace.
    external_crates: Vec<&'data ExternalCrate<'data>>,
    /// Name of the workspace.
    name: Option<String>,
    /// Path where the workspace is being created.
    path: &'data std::path::Path,
    /// Version of the workspace.
    version: (u8, u8, u8),
    /// Edition of the workspace.
    edition: u16,
}

use lazy_static::lazy_static;

lazy_static! {
    /// Default path for the workspace.
    pub static ref DEFAULT_WORKSPACE_PATH: std::path::PathBuf = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("synql_workspace");
}

impl Default for WorkspaceBuilder<'_> {
    fn default() -> Self {
        Self {
            external_crates: Vec::new(),
            name: None,
            path: DEFAULT_WORKSPACE_PATH.as_path(),
            version: (0, 1, 0),
            edition: 2024,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `Workspace` struct.
pub enum WorkspaceAttribute {
    /// External crates made available within the workspace.
    ExternalCrates,
    /// Name of the workspace.
    Name,
    /// Path of the workspace.
    Path,
    /// Version of the workspace.
    Version,
    /// Edition of the workspace.
    Edition,
}

impl Display for WorkspaceAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkspaceAttribute::ExternalCrates => write!(f, "external_crates"),
            WorkspaceAttribute::Name => write!(f, "name"),
            WorkspaceAttribute::Path => write!(f, "path"),
            WorkspaceAttribute::Version => write!(f, "version"),
            WorkspaceAttribute::Edition => write!(f, "edition"),
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
                write!(f, "A crate with the same name has already been added to the workspace")
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

impl<'data> WorkspaceBuilder<'data> {
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

    /// Sets the path where the workspace is being created.
    ///
    /// # Arguments
    /// * `path` - The path where the workspace is being created.
    pub fn path(mut self, path: &'data std::path::Path) -> Self {
        self.path = path;
        self
    }

    /// Sets the version of the workspace.
    ///
    /// # Arguments
    /// * `major` - The major version number.
    /// * `minor` - The minor version number.
    /// * `patch` - The patch version number.
    pub fn version(mut self, major: u8, minor: u8, patch: u8) -> Self {
        self.version = (major, minor, patch);
        self
    }

    /// Sets the edition of the workspace.
    ///
    /// # Arguments
    /// * `edition` - The Rust edition year (e.g., 2021, 2024).
    pub fn edition(mut self, edition: u16) -> Self {
        self.edition = edition;
        self
    }

    /// Adds an external crate to the workspace.
    ///
    /// # Arguments
    /// * `external_crate` - The external crate to add.
    pub fn external_crate(
        mut self,
        external_crate: &'data ExternalCrate<'data>,
    ) -> Result<Self, WorkspaceBuilderError> {
        if self.external_crates.contains(&external_crate) {
            return Err(WorkspaceBuilderError::DuplicatedCrateName);
        }
        self.external_crates.push(external_crate);
        Ok(self)
    }

    /// Adds the `std` external crate to the workspace.
    pub fn std(self) -> Result<Self, WorkspaceBuilderError> {
        self.external_crate(ExternalCrate::std())
    }

    /// Adds the core external crate to the workspace.
    pub fn core(self) -> Result<Self, WorkspaceBuilderError> {
        self.external_crate(ExternalCrate::core())
    }

    /// Adds the diesel external crate to the workspace.
    pub fn diesel(self) -> Result<Self, WorkspaceBuilderError> {
        self.external_crate(ExternalCrate::diesel())
    }

    /// Adds the serde external crate to the workspace.
    pub fn serde(self) -> Result<Self, WorkspaceBuilderError> {
        self.external_crate(ExternalCrate::serde())
    }

    /// Adds multiple external crates to the workspace.
    ///
    /// # Arguments
    /// * `external_crates` - The external crates to add.
    pub fn external_crates<I>(mut self, external_crates: I) -> Result<Self, WorkspaceBuilderError>
    where
        I: IntoIterator<Item = &'data ExternalCrate<'data>>,
    {
        for external_crate in external_crates {
            self = self.external_crate(external_crate)?;
        }
        Ok(self)
    }
}

impl Attributed for WorkspaceBuilder<'_> {
    type Attribute = WorkspaceAttribute;
}

impl IsCompleteBuilder for WorkspaceBuilder<'_> {
    fn is_complete(&self) -> bool {
        self.name.is_some()
    }
}

impl<'data> Builder for WorkspaceBuilder<'data> {
    type Error = BuilderError<WorkspaceAttribute>;
    type Object = Workspace<'data>;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Workspace {
            external_crates: self.external_crates,
            name: self.name.ok_or(BuilderError::IncompleteBuild(WorkspaceAttribute::Name))?,
            path: self.path,
            version: self.version,
            edition: self.edition,
            internal_crates: Vec::new(), // Internal crates are added later in the process.
        })
    }
}
