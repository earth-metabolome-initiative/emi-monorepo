//! Submodule defining a builder for the `Workspace` struct.

use std::path::PathBuf;

use crate::structs::{ExternalCrate, Workspace};

/// Builder for the `Workspace` struct.
pub struct WorkspaceBuilder {
    /// External crates made available within the workspace.
    external_crates: Vec<ExternalCrate>,
    /// Name of the workspace.
    name: String,
    /// Path where the workspace is being created.
    path: PathBuf,
    /// Version of the workspace.
    version: (u8, u8, u8),
    /// Edition of the workspace.
    edition: u16,
}

impl Default for WorkspaceBuilder {
    fn default() -> Self {
        Self {
            external_crates: Vec::new(),
            name: "synql-workspace".to_string(),
            path: PathBuf::from("synql_workspace"),
            version: (0, 1, 0),
            edition: 2024,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
/// Enumeration of errors that can occur during the building of a
/// `Workspace`.
pub enum WorkspaceBuilderError {
    #[error("The name of the workspace is invalid")]
    /// The name of the workspace is invalid.
    InvalidName,
}

impl WorkspaceBuilder {
    /// Sets the name of the workspace.
    ///
    /// # Arguments
    /// * `name` - The name of the workspace.
    pub fn name<S: ToString + ?Sized>(mut self, name: &S) -> Result<Self, WorkspaceBuilderError> {
        let name = name.to_string();
        if name.trim().is_empty() || name.contains(' ') {
            return Err(WorkspaceBuilderError::InvalidName);
        }
        self.name = name;
        Ok(self)
    }

    /// Sets the path where the workspace is being created.
    ///
    /// # Arguments
    /// * `path` - The path where the workspace is being created.
    pub fn path(mut self, path: std::path::PathBuf) -> Self {
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
    pub fn external_crate(mut self, external_crate: ExternalCrate) -> Self {
        if !self.external_crates.contains(&external_crate) {
            self.external_crates.push(external_crate);
        }
        self
    }

    /// Adds the `std` external crate to the workspace.
    pub fn std(self) -> Self {
        self.external_crate(ExternalCrate::std())
    }

    /// Adds the `chrono` external crate to the workspace.
    pub fn chrono(self) -> Self {
        self.external_crate(ExternalCrate::chrono())
    }

    /// Adds the `serde` external crate to the workspace.
    pub fn serde(self) -> Self {
        self.external_crate(ExternalCrate::serde())
    }

    /// Adds the `diesel_builders` external crate to the workspace.
    pub fn diesel_builders(self) -> Self {
        self.external_crate(ExternalCrate::diesel_builders())
    }

    /// Adds the core external crate to the workspace.
    pub fn core(self) -> Self {
        self.external_crate(ExternalCrate::core())
    }

    /// Adds the diesel external crate to the workspace.
    pub fn diesel(self) -> Self {
        self.external_crate(ExternalCrate::diesel())
    }

    /// Adds the `postgis-diesel` external crate to the workspace.
    pub fn postgis_diesel(self) -> Self {
        self.external_crate(ExternalCrate::postgis_diesel()).diesel()
    }

    /// Adds the `pgrx_validation` external crate to the workspace.
    pub fn pgrx_validation(self) -> Self {
        self.external_crate(ExternalCrate::pgrx_validation())
    }

    /// Adds the `rosetta_uuid` external crate to the workspace.
    pub fn rosetta_uuid(self) -> Self {
        self.external_crate(ExternalCrate::rosetta_uuid())
    }

    /// Adds multiple external crates to the workspace.
    ///
    /// # Arguments
    /// * `external_crates` - The external crates to add.
    pub fn external_crates<I>(mut self, external_crates: I) -> Self
    where
        I: IntoIterator<Item = ExternalCrate>,
    {
        for external_crate in external_crates {
            self = self.external_crate(external_crate);
        }
        self
    }
}

impl From<WorkspaceBuilder> for Workspace {
    fn from(builder: WorkspaceBuilder) -> Self {
        Workspace {
            external_crates: builder.external_crates,
            name: builder.name,
            path: builder.path,
            version: builder.version,
            edition: builder.edition,
        }
    }
}
