//! Submodule defining the `SQLWorkspace` trait which allows to generate the
//! `diesel` workspace from a SQL schema, based on `sql_traits`.

use std::path::Path;

mod builder;
mod write_crate_lib;
mod write_crate_toml;
mod write_sink_crate_lib;
mod write_sink_crate_toml;
pub use builder::SynQLBuilder;
use sql_relations::prelude::TableLike;
use time_requirements::{prelude::TimeTracker, task::Task};

use crate::{
    structs::{ExternalCrate, Workspace, external_crate::MaximalNumberOfColumns},
    traits::{SynQLDatabaseLike, table::TableSynLike},
};

/// Struct representing a SQL workspace.
pub struct SynQL<'db, DB: SynQLDatabaseLike> {
    /// The underlying database which will be used to generate the workspace.
    database: &'db DB,
    /// The path to the workspace.
    path: &'db Path,
    /// The path inside the workspace where the crates will be created.
    crate_base_path: &'db Path,
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
    /// Whether to also generate a crate which imports all the table crates.
    sink_crate_name: Option<String>,
    /// External rust crates to include in the workspace.
    external_crates: Vec<ExternalCrate>,
    /// Whether to clear workspace directory if it already exists.
    clear_existing: bool,
    /// Additional workspace members.
    members: Vec<&'db Path>,
}

impl<'db, DB: SynQLDatabaseLike> SynQL<'db, DB> {
    /// Create a new `SynQL` instance from a given database.
    #[must_use]
    #[inline]
    pub fn new(database: &'db DB, path: &'db Path) -> SynQLBuilder<'db, DB> {
        Self::new_with_crate_base_path(database, path, Path::new("."))
    }

    /// Create a new `SynQL` instance from a given database and crate base path.
    #[must_use]
    #[inline]
    pub fn new_with_crate_base_path(
        database: &'db DB,
        path: &'db Path,
        crate_base_path: &'db Path,
    ) -> SynQLBuilder<'db, DB> {
        SynQLBuilder::new(database, path, crate_base_path)
    }

    fn skip_table(&self, table: &DB::Table) -> bool {
        for deny_table in &self.deny_list {
            if table.depends_on(self.database, deny_table) {
                return true;
            }
        }
        false
    }

    /// Writes the workspace TOML.
    ///
    /// # Errors
    ///
    /// Returns an `std::io::Error` if writing to the file fails.
    pub fn write_toml(&self, workspace: &Workspace) -> std::io::Result<()> {
        use std::io::Write;

        let toml_path = self.path.join("Cargo.toml");
        let mut buffer = std::fs::File::create(toml_path)?;

        // Write [workspace] section
        writeln!(buffer, "[workspace]")?;
        writeln!(buffer, "resolver = \"2\"")?;

        // Write members array
        let mut wrote = false;
        write!(buffer, "members = [")?;

        for member in &self.members {
            if wrote {
                write!(buffer, ", ")?;
            }
            write!(buffer, "\"{}\"", member.display())?;
            wrote = true;
        }

        for table in self.database.tables() {
            if self.skip_table(table) {
                continue;
            }
            if wrote {
                write!(buffer, ", ")?;
            }

            write!(buffer, "\"{}\"", table.crate_relative_path(workspace).display())?;
            wrote = true;
        }

        if let Some(sink_crate_name) = &self.sink_crate_name {
            if wrote {
                write!(buffer, ", ")?;
            }
            write!(buffer, "\"{}\"", workspace.crate_base_path().join(sink_crate_name).display())?;
        }

        writeln!(buffer, "]")?;
        writeln!(buffer)?;

        // Write [workspace.package] section
        writeln!(buffer, "[workspace.package]")?;
        writeln!(buffer, "edition = \"{}\"", self.edition)?;
        writeln!(buffer)?;

        // Write [workspace.dependencies] section
        writeln!(buffer, "[workspace.dependencies]")?;

        // Write internal crate dependencies
        for table in self.database.tables() {
            if self.skip_table(table) {
                continue;
            }
            writeln!(
                buffer,
                "{crate_name} = {{ path = \"{crate_path}\" }}",
                crate_name = table.crate_name(workspace),
                crate_path = table.crate_relative_path(workspace).display(),
            )?;
        }

        // Write external dependencies
        for external_crate in workspace.external_crates() {
            if !external_crate.is_dependency() {
                continue;
            }

            let dep_name = external_crate.name();
            write!(buffer, "{dep_name} = {{ ")?;

            let mut parts = Vec::new();

            if let Some(version) = external_crate.version() {
                parts.push(format!("version = \"{version}\""));
            }

            if let Some((repository, branch)) = external_crate.git() {
                parts.push(format!("git = \"{repository}\""));
                parts.push(format!("branch = \"{branch}\""));
            }

            let features = external_crate.features();
            if !features.is_empty() {
                let features_str =
                    features.iter().map(|f| format!("\"{f}\"")).collect::<Vec<_>>().join(", ");
                parts.push(format!("features = [{features_str}]"));
            }

            write!(buffer, "{}", parts.join(", "))?;
            writeln!(buffer, " }}")?;
        }
        writeln!(buffer)?;

        // Write [workspace.lints.rust] section
        writeln!(buffer, "[workspace.lints.rust]")?;
        writeln!(buffer, "missing_docs = \"forbid\"")?;
        writeln!(buffer, "unused_macro_rules = \"forbid\"")?;
        writeln!(buffer, "unused_doc_comments = \"forbid\"")?;
        writeln!(buffer, "unconditional_recursion = \"forbid\"")?;
        writeln!(buffer, "unreachable_patterns = \"forbid\"")?;
        writeln!(buffer, "unused_import_braces = \"forbid\"")?;
        writeln!(buffer, "unused_must_use = \"forbid\"")?;
        writeln!(buffer, "deprecated = \"deny\"")?;
        writeln!(buffer)?;

        // Write [workspace.lints.rustdoc] section
        writeln!(buffer, "[workspace.lints.rustdoc]")?;
        writeln!(buffer, "broken_intra_doc_links = \"forbid\"")?;
        writeln!(buffer, "bare_urls = \"forbid\"")?;
        writeln!(buffer, "invalid_codeblock_attributes = \"forbid\"")?;
        writeln!(buffer, "invalid_html_tags = \"forbid\"")?;
        writeln!(buffer, "missing_crate_level_docs = \"forbid\"")?;
        writeln!(buffer, "unescaped_backticks = \"forbid\"")?;
        writeln!(buffer, "redundant_explicit_links = \"forbid\"")?;
        writeln!(buffer, "invalid_rust_codeblocks = \"forbid\"")?;

        Ok(())
    }

    /// Executes the workspace generation.
    ///
    /// # Errors
    ///
    /// Returns an error if the workspace cannot be written to disk.
    pub fn generate(&self) -> Result<TimeTracker, crate::Error> {
        let maximum_number_of_columns: MaximalNumberOfColumns = self
            .database
            .tables()
            .filter_map(|table| {
                if self.skip_table(table) {
                    None
                } else {
                    Some(table.number_of_columns(self.database))
                }
            })
            .max()
            .unwrap_or(0)
            .try_into()?;

        let workspace: Workspace = Workspace::new()
            .path(self.path.to_path_buf())
            .crate_base_path(self.crate_base_path.to_path_buf())
            .name(self.name.as_deref().unwrap_or_else(|| self.database.catalog_name()))
            .expect("Invalid workspace name")
            .external_crates(self.external_crates.iter().cloned())
            .rosetta_timestamp()
            .core()
            .std()
            .pgrx_validation()
            .serde()
            .serde_json()
            .validation_errors()
            .postgis_diesel(maximum_number_of_columns)
            .diesel_builders(maximum_number_of_columns)
            .rosetta_uuid()
            .version(self.version.0, self.version.1, self.version.2)
            .edition(self.edition)
            .into();

        if self.clear_existing {
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
        }

        let mut time_tracker = TimeTracker::new("SQL Workspace Generation");

        for table in self.database.table_dag() {
            if self.skip_table(table) {
                continue;
            }

            // Create the directory for the crate
            let crate_path = table.crate_absolute_path(&workspace);
            std::fs::create_dir_all(&crate_path)?;

            let writing_toml = Task::new("writing_crate_toml");
            self.write_crate_toml(table, &workspace)?;
            time_tracker.add_or_extend_completed_task(writing_toml);
            let writing_lib = Task::new("writing_crate_lib");
            self.write_crate_lib(table, &workspace)?;
            time_tracker.add_or_extend_completed_task(writing_lib);
        }

        if let Some(sink_crate_name) = &self.sink_crate_name {
            let sink_crate_path =
                workspace.path().join(workspace.crate_base_path()).join(sink_crate_name);
            std::fs::create_dir_all(&sink_crate_path)?;

            let writing_sink_toml = Task::new("writing_sink_crate_toml");
            self.write_sink_crate_toml(&workspace, sink_crate_name, &sink_crate_path)?;
            time_tracker.add_or_extend_completed_task(writing_sink_toml);

            let writing_sink_lib = Task::new("writing_sink_crate_lib");
            self.write_sink_crate_lib(&workspace, sink_crate_name, &sink_crate_path)?;
            time_tracker.add_or_extend_completed_task(writing_sink_lib);
        }

        if self.generate_workspace_toml {
            let workspace_toml_task = Task::new("workspace_toml");
            self.write_toml(&workspace)?;
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
