//! Submodule defining the `SQLWorkspace` trait which allows to generate the
//! `diesel` workspace from a SQL schema, based on `sql_traits`.

use std::path::Path;

mod builder;
mod write_crate_lib;
mod write_crate_toml;
pub use builder::SynQLBuilder;
use sql_relations::prelude::TableLike;
use time_requirements::{prelude::TimeTracker, task::Task};

use crate::{
    structs::{ExternalCrate, Workspace},
    traits::{SynQLDatabaseLike, table::TableSynLike},
};

/// Struct representing a SQL workspace.
pub struct SynQL<'db, DB: SynQLDatabaseLike> {
    /// The underlying database which will be used to generate the workspace.
    database: &'db DB,
    /// The path to the workspace.
    path: &'db Path,
    /// Optional name of the workspace.
    name: Option<String>,
    /// List of tables to be excluded from the workspace, which also imply
    /// excluding all of the tables that depend on them via foreign keys.
    deny_list: Vec<&'db DB::Table>,
    /// Version of the generated workspace.
    version: (u8, u8, u8),
    /// Edition of the generated workspace.
    edition: u16,
    /// Whether to also generate the workspace TOML.
    generate_workspace_toml: bool,
    /// Whether to also generate the rustfmt configuration file.
    generate_rustfmt: bool,
    /// External rust crates to include in the workspace.
    external_crates: Vec<ExternalCrate>,
}

impl<'db, DB: SynQLDatabaseLike> SynQL<'db, DB> {
    /// Create a new `SynQL` instance from a given database.
    #[must_use]
    #[inline]
    pub fn new(database: &'db DB, path: &'db Path) -> SynQLBuilder<'db, DB> {
        SynQLBuilder::new(database, path)
    }

    fn skip_table(&self, table: &DB::Table) -> bool {
        for deny_table in &self.deny_list {
            if table.depends_on(self.database, deny_table) {
                return true;
            }
        }
        false
    }

    /// Executes the workspace generation.
    ///
    /// # Errors
    ///
    /// Returns an error if the workspace cannot be written to disk.
    pub fn generate(&self) -> Result<TimeTracker, crate::Error> {
        let workspace: Workspace = Workspace::new()
            .path(self.path.to_path_buf())
            .name(self.name.as_deref().unwrap_or_else(|| self.database.catalog_name()))
            .expect("Invalid workspace name")
            .external_crates(self.external_crates.iter().cloned())
            .rosetta_timestamp()
            .core()
            .std()
            .pgrx_validation()
            .serde()
            .validation_errors()
            .postgis_diesel()
            .diesel_builders()
            .rosetta_uuid()
            .version(self.version.0, self.version.1, self.version.2)
            .edition(self.edition)
            .into();

        // Clear up any directory or file that may already exist at the workspace path
        if workspace.path().exists() {
            // We remove all contents of the directory, but we do not remove the directory
            // itself
            for entry in std::fs::read_dir(workspace.path())? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    std::fs::remove_dir_all(path)?;
                } else {
                    std::fs::remove_file(path)?;
                }
            }
        }

        let mut time_tracker = TimeTracker::new("SQL Workspace Generation");

        for table in self.database.table_dag() {
            if self.skip_table(table) {
                continue;
            }

            // Create the directory for the crate
            let crate_path = table.crate_path(&workspace);
            std::fs::create_dir_all(&crate_path)?;

            let writing_toml = Task::new("writing_crate_toml");
            self.write_crate_toml(table, &workspace)?;
            time_tracker.add_or_extend_completed_task(writing_toml);
            let writing_lib = Task::new("writing_crate_lib");
            self.write_crate_lib(table, &workspace)?;
            time_tracker.add_or_extend_completed_task(writing_lib);
        }

        if self.generate_workspace_toml {
            let workspace_toml_task = Task::new("workspace_toml");
            workspace.write_toml(self.database)?;
            time_tracker.add_or_extend_completed_task(workspace_toml_task);
        }

        if self.generate_rustfmt {
            let workspace_rustfmt_task = Task::new("workspace_rustfmt");
            workspace.write_rustfmt()?;
            time_tracker.add_or_extend_completed_task(workspace_rustfmt_task);
        }

        Ok(time_tracker)
    }
}
