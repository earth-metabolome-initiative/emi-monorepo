//! Submodule defining a `Workspace` struct representing a Cargo
//! workspace.

mod builder;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub use builder::WorkspaceBuilder;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use syn::Type;

use crate::{
    structs::{
        ExternalCrate, ExternalFunctionRef, InternalCrate,
        external_crate::{ExternalMacroRef, ExternalTraitRef, ExternalTypeRef},
    },
    traits::ExternalDependencies,
};

#[derive(Debug, Clone)]
/// Struct defining a Cargo workspace.
pub struct Workspace {
    /// External crates made available within the workspace.
    external_crates: Vec<Arc<ExternalCrate>>,
    /// Name of the workspace.
    name: String,
    /// Path where the workspace is being created.
    path: PathBuf,
    /// Version of the workspace.
    version: (u8, u8, u8),
    /// Edition of the workspace.
    edition: u16,
    /// Internal crates created within the workspace.
    internal_crates: Vec<Arc<InternalCrate>>,
}

impl Workspace {
    /// Inizializes a new `WorkspaceBuilder`.
    pub fn new() -> WorkspaceBuilder {
        WorkspaceBuilder::default()
    }

    /// Returns the version tuple of the workspace.
    #[inline]
    pub fn version(&self) -> (u8, u8, u8) {
        self.version
    }

    /// Returns the edition of the workspace.
    #[inline]
    pub fn edition(&self) -> u16 {
        self.edition
    }

    /// Returns the path where the workspace is being created.
    #[inline]
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    /// Adds a new internal crate to the workspace.
    pub fn add_internal_crate(&mut self, internal_crate: InternalCrate) {
        self.internal_crates.push(Arc::new(internal_crate));
    }

    /// Returns the internal crate with the given name, if any.
    pub fn internal_crate(&self, name: &str) -> Option<&Arc<InternalCrate>> {
        for internal_crate in &self.internal_crates {
            if internal_crate.name() == name {
                return Some(internal_crate);
            }
        }
        None
    }

    /// Returns the external macro ref corresponding to the provided name, if
    /// any.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the external macro.
    pub fn external_macro(&self, name: &str) -> Option<ExternalMacroRef> {
        for ext_crate in &self.external_crates {
            if let Some(ext_macro) = ext_crate.external_macro(name) {
                return Some(ext_macro);
            }
        }
        None
    }

    /// Returns the external trait ref corresponding to the provided name, if
    /// any.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the external trait.
    pub fn external_trait(&self, name: &str) -> Option<ExternalTraitRef> {
        for ext_crate in &self.external_crates {
            if let Some(ext_trait) = ext_crate.external_trait_ref(name) {
                return Some(ext_trait);
            }
        }
        None
    }

    /// Returns the external type ref corresponding to the provided Postgres
    /// name, if any.
    ///
    /// # Arguments
    /// * `postgres_type` - A string slice representing the postgres type.
    pub fn external_postgres_type(&self, postgres_type: &str) -> Option<ExternalTypeRef> {
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
    pub fn external_type(&self, ident: &Type) -> Option<ExternalTypeRef> {
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
    pub fn external_function(&self, name: &str) -> Option<ExternalFunctionRef> {
        for ext_crate in &self.external_crates {
            if let Some(ext_function) = ext_crate.external_function_ref(name) {
                return Some(ext_function);
            }
        }
        None
    }

    /// Writes the formatting rules for the workspace.
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
        writeln!(buffer, "normalize_doc_attributes = false")?;
        writeln!(buffer, "force_multiline_blocks = true")?;
        writeln!(buffer, "fn_single_line = false")?;
        writeln!(buffer, "where_single_line = false")?;

        Ok(())
    }

    /// Writes the workspace TOML.
    pub fn write_toml(&self) -> std::io::Result<()> {
        use std::io::Write;

        let toml_path = self.path.join("Cargo.toml");
        let mut buffer = std::fs::File::create(toml_path)?;

        // Write [workspace] section
        writeln!(buffer, "[workspace]")?;
        writeln!(buffer, "resolver = \"2\"")?;

        // Write members array
        write!(buffer, "members = [")?;
        for (i, internal_crate) in self.internal_crates.iter().enumerate() {
            if i > 0 {
                write!(buffer, ", ")?;
            }
            write!(buffer, "\"{}\"", internal_crate.name())?;
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
        for internal_crate in &self.internal_crates {
            writeln!(
                buffer,
                "{} = {{ path = \"{}\" }}",
                internal_crate.name(),
                internal_crate.name()
            )?;
        }

        // Collect and sort external dependencies
        let mut external_deps: Vec<_> = self
            .internal_crates
            .iter()
            .flat_map(|internal_crate| internal_crate.external_dependencies())
            .collect();

        external_deps.sort_unstable();
        external_deps.dedup();

        // Write external dependencies
        for external_crate in external_deps {
            if !external_crate.is_dependency() {
                continue;
            }

            let dep_name = external_crate.name();
            write!(buffer, "{} = {{ ", dep_name)?;

            let mut parts = Vec::new();

            if let Some(version) = external_crate.version() {
                parts.push(format!("version = \"{}\"", version));
            }

            if let Some((repository, branch)) = external_crate.git() {
                parts.push(format!("git = \"{}\"", repository));
                parts.push(format!("branch = \"{}\"", branch));
            }

            let features = external_crate.features();
            if !features.is_empty() {
                let features_str =
                    features.iter().map(|f| format!("\"{}\"", f)).collect::<Vec<_>>().join(", ");
                parts.push(format!("features = [{}]", features_str));
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

    /// Writes the workspace to disk.
    pub fn write_to_disk(&self) -> std::io::Result<()> {
        // First, we eliminate all existing files in the workspace path.
        if self.path.exists() {
            // We remove all contents of the directory.
            for entry in std::fs::read_dir(self.path())? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    std::fs::remove_dir_all(path)?;
                } else {
                    std::fs::remove_file(path)?;
                }
            }
        }
        // Then, we create the workspace directory.
        std::fs::create_dir_all(self.path())?;
        // And we start writing each internal crate to disk.
        self.internal_crates
            .par_iter()
            .map(|internal_crate: &Arc<InternalCrate>| internal_crate.write_to_disk(self))
            .collect::<Result<Vec<()>, std::io::Error>>()?;
        Ok(())
    }
}
