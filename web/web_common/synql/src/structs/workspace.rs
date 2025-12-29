//! Submodule defining a `Workspace` struct representing a Cargo
//! workspace.

mod builder;
use std::path::{Path, PathBuf};
mod core_types;

pub use builder::WorkspaceBuilder;
use syn::Type;

use crate::structs::{ExternalCrate, ExternalFunctionRef, ExternalTypeRef};

#[derive(Debug, Clone)]
/// Struct defining a Cargo workspace.
pub struct Workspace {
    /// External crates made available within the workspace.
    external_crates: Vec<ExternalCrate>,
    /// Name of the workspace.
    name: String,
    /// Path where the workspace is being created.
    path: PathBuf,
    /// Path inside the workspace where the crates will be created.
    crate_base_path: PathBuf,
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
    /// Returns the path inside the workspace where the crates will be created.
    pub fn crate_base_path(&self) -> &Path {
        self.crate_base_path.as_path()
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

    /// Iterates over the external crates in the workspace.
    #[must_use]
    pub fn external_crates(&self) -> impl Iterator<Item = &ExternalCrate> {
        self.external_crates.iter()
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
}
