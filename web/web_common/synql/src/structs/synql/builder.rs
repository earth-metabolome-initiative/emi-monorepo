//! Submodule providing a builder for the `SynQL` struct.

use std::path::Path;

use super::SynQL;
use crate::{structs::ExternalCrate, traits::SynQLDatabaseLike};

/// Struct to build `SynQL` instances.
pub struct SynQLBuilder<'db, DB: SynQLDatabaseLike> {
    database: &'db DB,
    path: &'db Path,
    crate_base_path: &'db Path,
    clear_existing: bool,
    name: Option<String>,
    deny_list: Vec<&'db DB::Table>,
    version: (u8, u8, u8),
    edition: u16,
    generate_workspace_toml: bool,
    generate_rustfmt: bool,
    sink_crate_name: Option<String>,
    external_crates: Vec<ExternalCrate>,
    /// Additional workspace members.
    members: Vec<&'db Path>,
}

impl<'db, DB: SynQLDatabaseLike> SynQLBuilder<'db, DB> {
    #[must_use]
    #[inline]
    /// Creates a new `SynQLBuilder` instance.
    pub fn new(database: &'db DB, path: &'db Path, crate_base_path: &'db Path) -> Self {
        SynQLBuilder {
            database,
            path,
            crate_base_path,
            clear_existing: false,
            name: None,
            deny_list: Vec::new(),
            version: (0, 1, 0),
            edition: 2024,
            generate_workspace_toml: false,
            generate_rustfmt: false,
            sink_crate_name: None,
            external_crates: Vec::new(),
            members: Vec::new(),
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

    #[must_use]
    #[inline]
    /// Sets to clear existing workspace directory if it already exists.
    pub fn clear_existing(mut self) -> Self {
        self.clear_existing = true;
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

    /// Sets to generate a sink crate which imports all the table crates.
    #[must_use]
    #[inline]
    pub fn sink_crate(mut self, sink_crate_name: &str) -> Self {
        self.sink_crate_name = Some(sink_crate_name.to_string());
        self
    }

    /// Adds a member path to the workspace.
    ///
    /// # Arguments
    /// * `member` - The member path to add.
    #[must_use]
    pub fn member<S: AsRef<Path> + ?Sized>(mut self, member: &'db S) -> Self {
        if !self.members.contains(&member.as_ref()) {
            self.members.push(member.as_ref());
        }
        self
    }

    /// Adds several member paths to the workspace.
    ///
    /// # Arguments
    /// * `members` - The member paths to add.
    #[must_use]
    pub fn members<I, S>(mut self, members: I) -> Self
    where
        I: IntoIterator<Item = &'db S> + 'db,
        S: AsRef<Path> + ?Sized + 'db,
    {
        for member in members {
            if !self.members.contains(&member.as_ref()) {
                self.members.push(member.as_ref());
            }
        }
        self
    }
}

impl<'db, DB: SynQLDatabaseLike> From<SynQLBuilder<'db, DB>> for SynQL<'db, DB> {
    fn from(builder: SynQLBuilder<'db, DB>) -> Self {
        SynQL {
            database: builder.database,
            clear_existing: builder.clear_existing,
            path: builder.path,
            crate_base_path: builder.crate_base_path,
            name: builder.name,
            deny_list: builder.deny_list,
            version: builder.version,
            edition: builder.edition,
            generate_workspace_toml: builder.generate_workspace_toml,
            generate_rustfmt: builder.generate_rustfmt,
            sink_crate_name: builder.sink_crate_name,
            external_crates: builder.external_crates,
            members: builder.members,
        }
    }
}
