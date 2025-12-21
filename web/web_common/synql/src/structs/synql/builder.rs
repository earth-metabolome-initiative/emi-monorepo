//! Submodule providing a builder for the `SynQL` struct.

use std::path::Path;

use super::SynQL;
use crate::{structs::ExternalCrate, traits::SynQLDatabaseLike};

/// Struct to build `SynQL` instances.
pub struct SynQLBuilder<'db, DB: SynQLDatabaseLike> {
    database: &'db DB,
    path: &'db Path,
    name: Option<String>,
    deny_list: Vec<&'db DB::Table>,
    version: (u8, u8, u8),
    edition: u16,
    generate_workspace_toml: bool,
    generate_rustfmt: bool,
    external_crates: Vec<ExternalCrate>,
}

impl<'db, DB: SynQLDatabaseLike> SynQLBuilder<'db, DB> {
    #[must_use]
    #[inline]
    /// Creates a new `SynQLBuilder` instance.
    pub fn new(database: &'db DB, path: &'db Path) -> Self {
        SynQLBuilder {
            database,
            path,
            name: None,
            deny_list: Vec::new(),
            version: (0, 1, 0),
            edition: 2024,
            generate_workspace_toml: false,
            generate_rustfmt: false,
            external_crates: Vec::new(),
        }
    }

    /// Sets the deny list for the `SynQL` instance.
    #[must_use]
    #[inline]
    pub fn deny_list(mut self, deny_list: Vec<&'db DB::Table>) -> Self {
        self.deny_list = deny_list;
        self
    }

    /// Adds a table to the deny list.
    #[must_use]
    #[inline]
    pub fn deny(mut self, table: &'db DB::Table) -> Self {
        self.deny_list.push(table);
        self
    }

    /// Sets the name of the workspace.
    #[must_use]
    #[inline]
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Sets the version for the `SynQL` instance.
    #[must_use]
    #[inline]
    pub fn version(mut self, major: u8, minor: u8, patch: u8) -> Self {
        self.version = (major, minor, patch);
        self
    }

    /// Sets the edition for the `SynQL` instance.
    #[must_use]
    #[inline]
    pub fn edition(mut self, edition: u16) -> Self {
        self.edition = edition;
        self
    }

    /// Sets to generate the workspace TOML.
    #[must_use]
    #[inline]
    pub fn generate_workspace_toml(mut self) -> Self {
        self.generate_workspace_toml = true;
        self
    }

    /// Adds an external crate to the workspace.
    #[must_use]
    #[inline]
    pub fn external_crate(mut self, external_crate: ExternalCrate) -> Self {
        self.external_crates.push(external_crate);
        self
    }

    /// Adds several external crates to the workspace.
    #[must_use]
    pub fn external_crates<I>(mut self, external_crates: I) -> Self
    where
        I: IntoIterator<Item = ExternalCrate>,
    {
        for external_crate in external_crates {
            self.external_crates.push(external_crate);
        }
        self
    }

    /// Sets to generate the rustfmt configuration file.
    #[must_use]
    #[inline]
    pub fn generate_rustfmt(mut self) -> Self {
        self.generate_rustfmt = true;
        self
    }
}

impl<'db, DB: SynQLDatabaseLike> From<SynQLBuilder<'db, DB>> for SynQL<'db, DB> {
    fn from(builder: SynQLBuilder<'db, DB>) -> Self {
        SynQL {
            database: builder.database,
            path: builder.path,
            name: builder.name,
            deny_list: builder.deny_list,
            version: builder.version,
            edition: builder.edition,
            generate_workspace_toml: builder.generate_workspace_toml,
            generate_rustfmt: builder.generate_rustfmt,
            external_crates: builder.external_crates,
        }
    }
}
