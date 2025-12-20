//! Submodule defining the `SQLWorkspace` trait which allows to generate the
//! `diesel` workspace from a SQL schema, based on `sql_traits`.

use std::{path::PathBuf, sync::Arc};


mod builder;
pub use builder::SynQLBuilder;
use time_requirements::{prelude::TimeTracker, task::Task};

use crate::{structs::ExternalCrate, traits::SynQLDatabaseLike};

/// Struct representing a SQL workspace.
pub struct SynQL<'a, DB: SynQLDatabaseLike> {
    /// The underlying database which will be used to generate the workspace.
    database: &'a DB,
    /// The path to the workspace.
    path: PathBuf,
    /// List of tables to be excluded from the workspace, which also imply
    /// excluding all of the tables that depend on them via foreign keys.
    deny_list: Vec<&'a DB::Table>,
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

impl<'a, DB: SynQLDatabaseLike> SynQL<'a, DB> {
    /// Create a new `SynQL` instance from a given database.
    #[must_use]
    #[inline]
    pub fn new() -> SynQLBuilder<'a, DB> {
        SynQLBuilder::default()
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
    pub fn generate(&self) -> std::io::Result<TimeTracker> {
        let mut workspace = Workspace::new()
            .path(self.path.clone())
            .name(&self.database.catalog_name())
            .expect("Invalid workspace name")
            .external_crates(self.external_crates.iter().cloned())
            .core()
            .std()
            .serde()
            .validation_errors()
            .pgrx_validation()
            .postgis_diesel()
            .rosetta_uuid()
            .version(self.version.0, self.version.1, self.version.2)
            .edition(self.edition)
            .build()
            .expect("Unable to build workspace");

        let mut time_tracker = TimeTracker::new("SQL Workspace Generation");

        for table in self.database.table_dag() {
            if self.skip_table(table) {
                continue;
            }

            todo!("Generate table model");
        }

        let workspace_write_task = Task::new("workspace_write_to_disk");
        workspace.write_to_disk()?;
        time_tracker.add_or_extend_completed_task(workspace_write_task);

        if self.generate_workspace_toml {
            let workspace_toml_task = Task::new("workspace_toml");
            workspace.write_toml()?;
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
