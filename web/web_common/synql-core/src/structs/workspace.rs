//! Submodule defining a `Workspace` struct representing a Cargo
//! workspace.

mod builder;
pub use builder::WorkspaceBuilder;

use crate::structs::{ExternalCrate, InternalCrate};

/// Struct defining a Cargo workspace.
pub struct Workspace {
    /// External crates made available within the workspace.
    external_crates: Vec<ExternalCrate>,
    /// Name of the workspace.
    name: String,
    /// Version of the workspace.
    version: (u8, u8, u8),
    /// Internal crates created within the workspace.
    internal_crates: Vec<InternalCrate>,
}

impl Workspace {
    /// Inizializes a new `WorkspaceBuilder`.
    pub fn new() -> WorkspaceBuilder {
        WorkspaceBuilder::default()
    }

    /// Returns the diesel [`Type`](syn::Type) corresponding to the
    /// provided postgres type.
    ///
    /// # Arguments
    ///
    /// * `pg_type` - A string slice representing the postgres type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use synql_core::prelude::*;
    /// use quote::ToTokens;
    /// let workspace = Workspace::new()
    ///    .name("my_workspace")?
    ///    .version(0, 1, 0)
    ///    .core()?
    ///    .build()?;
    ///
    /// let diesel_type = workspace.diesel_type("int4").expect("Type not found").clone();
    /// assert_eq!(diesel_type.to_token_stream().to_string(), "diesel :: sql_types :: Integer");
    /// # Ok(())
    /// # }
    /// ```
    pub fn diesel_type(&self, pg_type: &str) -> Option<&syn::Type> {
        for ext_crate in &self.external_crates {
            if let Some(compatible_type) = ext_crate.compatible_type(pg_type) {
                return Some(compatible_type.diesel_type());
            }
        }
        None
    }
}
