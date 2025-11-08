//! Submodule providing a builder for the `SynQL` struct.

use std::{fmt::Display, path::Path, sync::Arc};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};
use synql_core::structs::ExternalCrate;

use super::SynQL;
use crate::traits::SynQLDatabaseLike;

/// Struct to build `SynQL` instances.
pub struct SynQLBuilder<'a, DB: SynQLDatabaseLike> {
    database: Option<&'a DB>,
    path: Option<&'a Path>,
    deny_list: Vec<&'a DB::Table>,
    version: (u8, u8, u8),
    edition: u16,
    generate_workspace_toml: bool,
    generate_rustfmt: bool,
    external_crates: Vec<Arc<ExternalCrate>>,
}

impl<'a, DB: SynQLDatabaseLike> Default for SynQLBuilder<'a, DB> {
    fn default() -> Self {
        SynQLBuilder {
            database: None,
            path: None,
            deny_list: Vec::new(),
            version: (0, 1, 0),
            edition: 2024,
            generate_workspace_toml: false,
            generate_rustfmt: false,
            external_crates: Vec::new(),
        }
    }
}

/// Attributes that can be set on the `SynQLBuilder`.
#[derive(Debug)]
pub enum SynQLAttribute {
    /// The database instance.
    Database,
    /// The path to the workspace.
    Path,
    /// The deny list of tables to exclude.
    DenyList,
    /// The version of the generated workspace.
    Version,
    /// The edition of the generated workspace.
    Edition,
    /// Whether to also generate the workspace TOML.
    GenerateWorkspaceToml,
    /// Whether to also generate the rustfmt configuration file.
    GenerateRustfmt,
}

impl Display for SynQLAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SynQLAttribute::Database => write!(f, "database"),
            SynQLAttribute::Path => write!(f, "path"),
            SynQLAttribute::DenyList => write!(f, "deny_list"),
            SynQLAttribute::Version => write!(f, "version"),
            SynQLAttribute::Edition => write!(f, "edition"),
            SynQLAttribute::GenerateWorkspaceToml => write!(f, "generate_workspace_toml"),
            SynQLAttribute::GenerateRustfmt => write!(f, "generate_rustfmt"),
        }
    }
}

impl<'a, DB: SynQLDatabaseLike> SynQLBuilder<'a, DB> {
    /// Creates a new `SynQLBuilder` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the database for the `SynQL` instance.
    pub fn database(mut self, database: &'a DB) -> Self {
        self.database = Some(database);
        self
    }

    /// Sets the path for the `SynQL` instance.
    pub fn path(mut self, path: &'a Path) -> Self {
        self.path = Some(path);
        self
    }

    /// Sets the deny list for the `SynQL` instance.
    pub fn deny_list(mut self, deny_list: Vec<&'a DB::Table>) -> Self {
        self.deny_list = deny_list;
        self
    }

    /// Adds a table to the deny list.
    pub fn deny(mut self, table: &'a DB::Table) -> Self {
        self.deny_list.push(table);
        self
    }

    /// Sets the version for the `SynQL` instance.
    pub fn version(mut self, major: u8, minor: u8, patch: u8) -> Self {
        self.version = (major, minor, patch);
        self
    }

    /// Sets the edition for the `SynQL` instance.
    pub fn edition(mut self, edition: u16) -> Self {
        self.edition = edition;
        self
    }

    /// Sets to generate the workspace TOML.
    pub fn generate_workspace_toml(mut self) -> Self {
        self.generate_workspace_toml = true;
        self
    }

    /// Adds an external crate to the workspace.
    pub fn external_crate(mut self, external_crate: Arc<ExternalCrate>) -> Self {
        self.external_crates.push(external_crate);
        self
    }

    /// Adds several external crates to the workspace.
    pub fn external_crates<I>(mut self, external_crates: I) -> Self
    where
        I: IntoIterator<Item = Arc<ExternalCrate>>,
    {
        for external_crate in external_crates {
            self.external_crates.push(external_crate);
        }
        self
    }

    /// Sets to generate the rustfmt configuration file.
    pub fn generate_rustfmt(mut self) -> Self {
        self.generate_rustfmt = true;
        self
    }
}

impl<'a, DB: SynQLDatabaseLike> Attributed for SynQLBuilder<'a, DB> {
    type Attribute = SynQLAttribute;
}

impl<'a, DB: SynQLDatabaseLike> IsCompleteBuilder for SynQLBuilder<'a, DB> {
    fn is_complete(&self) -> bool {
        self.database.is_some() && self.path.is_some()
    }
}

impl<'a, DB: SynQLDatabaseLike> Builder for SynQLBuilder<'a, DB> {
    type Error = BuilderError<SynQLAttribute>;
    type Object = SynQL<'a, DB>;

    fn build(self) -> Result<Self::Object, Self::Error> {
        let database =
            self.database.ok_or(BuilderError::IncompleteBuild(SynQLAttribute::Database))?;
        let path = self.path.ok_or(BuilderError::IncompleteBuild(SynQLAttribute::Path))?;

        Ok(SynQL {
            database,
            path,
            deny_list: self.deny_list,
            version: self.version,
            edition: self.edition,
            generate_workspace_toml: self.generate_workspace_toml,
            generate_rustfmt: self.generate_rustfmt,
            external_crates: self.external_crates,
        })
    }
}
