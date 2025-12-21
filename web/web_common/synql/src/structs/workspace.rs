//! Submodule defining a `Workspace` struct representing a Cargo
//! workspace.

mod builder;
use std::path::{Path, PathBuf};
mod core_types;

pub use builder::WorkspaceBuilder;
use syn::Type;

use crate::{
    structs::{ExternalCrate, ExternalFunctionRef, ExternalTypeRef},
    traits::{SynQLDatabaseLike, TableSynLike},
};

#[derive(Debug, Clone)]
/// Struct defining a Cargo workspace.
pub struct Workspace {
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

impl Workspace {
    /// Inizializes a new `WorkspaceBuilder`.
    #[must_use]
    pub fn new() -> WorkspaceBuilder {
        WorkspaceBuilder::default()
    }

    /// Returns the version tuple of the workspace.
    #[inline]
    #[must_use]
    pub fn version(&self) -> (u8, u8, u8) {
        self.version
    }

    /// Returns the edition of the workspace.
    #[inline]
    #[must_use]
    pub fn edition(&self) -> u16 {
        self.edition
    }

    /// Returns the path where the workspace is being created.
    #[inline]
    #[must_use]
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    #[inline]
    #[must_use]
    /// Returns the name of the workspace.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the external type ref corresponding to the provided Postgres
    /// name, if any.
    ///
    /// # Arguments
    /// * `postgres_type` - A string slice representing the postgres type.
    #[must_use]
    pub fn external_postgres_type(&self, postgres_type: &str) -> Option<ExternalTypeRef<'_>> {
        for ext_crate in &self.external_crates {
            if let Some(ext_type) = ext_crate.external_postgres_type(postgres_type) {
                return Some(ext_type);
            }
        }
        None
    }

    /// Returns the external type ref corresponding to the provided name, if
    /// any.
    ///
    /// # Arguments
    /// * `ident` - A reference to the type.
    #[must_use]
    pub fn external_type(&self, ident: &Type) -> Option<ExternalTypeRef<'_>> {
        for ext_crate in &self.external_crates {
            if let Some(ext_type) = ext_crate.external_type(ident) {
                return Some(ext_type);
            }
        }
        None
    }

    /// Returns the external function ref corresponding to the provided name, if
    /// any.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the external
    ///   function.
    #[must_use]
    pub fn external_function(&self, name: &str) -> Option<ExternalFunctionRef<'_>> {
        for ext_crate in &self.external_crates {
            if let Some(ext_function) = ext_crate.external_function_ref(name) {
                return Some(ext_function);
            }
        }
        None
    }

    /// Writes the formatting rules for the workspace.
    ///
    /// # Errors
    ///
    /// Returns an `std::io::Error` if writing to the file fails.
    pub fn write_rustfmt(&self) -> std::io::Result<()> {
        use std::io::Write;

        let rustfmt_path = self.path.join("rustfmt.toml");
        let mut buffer = std::fs::File::create(rustfmt_path)?;

        writeln!(buffer, "edition = \"{}\"", self.edition)?;
        writeln!(buffer, "max_width = 100")?;
        writeln!(buffer, "use_small_heuristics = \"Max\"")?;
        writeln!(buffer, "reorder_imports = true")?;
        writeln!(buffer, "group_imports = \"StdExternalCrate\"")?;
        writeln!(buffer, "imports_granularity = \"Crate\"")?;
        writeln!(buffer, "reorder_modules = true")?;
        writeln!(buffer, "wrap_comments = true")?;
        writeln!(buffer, "format_code_in_doc_comments = true")?;
        writeln!(buffer, "comment_width = 80")?;
        writeln!(buffer, "normalize_comments = true")?;
        writeln!(buffer, "normalize_doc_attributes = true")?;
        writeln!(buffer, "force_multiline_blocks = true")?;
        writeln!(buffer, "fn_single_line = false")?;
        writeln!(buffer, "where_single_line = false")?;

        Ok(())
    }

    /// Writes the workspace TOML.
    ///
    /// # Errors
    ///
    /// Returns an `std::io::Error` if writing to the file fails.
    pub fn write_toml<DB: SynQLDatabaseLike>(&self, db: &DB) -> std::io::Result<()> {
        use std::io::Write;

        let toml_path = self.path.join("Cargo.toml");
        let mut buffer = std::fs::File::create(toml_path)?;

        // Write [workspace] section
        writeln!(buffer, "[workspace]")?;
        writeln!(buffer, "resolver = \"2\"")?;

        // Write members array
        write!(buffer, "members = [")?;
        for (i, table) in db.tables().enumerate() {
            if i > 0 {
                write!(buffer, ", ")?;
            }
            write!(buffer, "\"{}\"", table.crate_name(self))?;
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
        for table in db.tables() {
            writeln!(
                buffer,
                "{crate_name} = {{ path = \"./{crate_name}\" }}",
                crate_name=table.crate_name(self),
            )?;
        }

        // Write external dependencies
        for external_crate in self.external_crates.iter() {
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
}
