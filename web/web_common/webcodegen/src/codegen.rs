//! Submodule providing the 'Codegen' struct for code generation.

use std::path::Path;

use diesel::PgConnection;
use prettyplease::unparse;
use proc_macro2::TokenStream;
use syn::File;
mod diesel_codegen;

use crate::{
    errors::{CodeGenerationError, WebCodeGenError},
    Table,
};

#[derive(Debug, Default)]
/// Struct for code generation.
pub struct Codegen<'a> {
    /// List of tables to ignore when generating code.
    tables_deny_list: Vec<&'a Table>,
    /// The output directory for the generated code.
    output_directory: Option<&'a Path>,
    /// Whether to make the code readable.
    beautify: bool,
    /// Whether to generate the diesel joinables.
    pub(super) enable_joinables: bool,
    /// Whether to generate the diesel allow_tables_to_appear_in_same_query.
    pub(super) enable_allow_tables_to_appear_in_same_query: bool,
}

impl<'a> Codegen<'a> {
    #[must_use]
    /// Adds a new table to the deny list.
    pub fn add_table_to_deny_list(mut self, table: &'a Table) -> Self {
        self.tables_deny_list.push(table);
        self
    }

    #[must_use]
    /// Sets the output directory for the generated code.
    pub fn set_output_directory(mut self, output_directory: &'a Path) -> Self {
        self.output_directory = Some(output_directory);
        self
    }

    #[must_use]
    /// Whether to generate the diesel joinables.
    pub fn enable_joinables(mut self) -> Self {
        self.enable_joinables = true;
        self
    }

    #[must_use]
    /// Whether to generate the diesel allow_tables_to_appear_in_same_query.
    pub fn enable_allow_tables_to_appear_in_same_query(mut self) -> Self {
        self.enable_allow_tables_to_appear_in_same_query = true;
        self
    }

    #[must_use]
    /// Whether to make the code beautified after generation.
    pub fn beautify(mut self) -> Self {
        self.beautify = true;
        self
    }

    /// Dispatches beautification for the provided TokenStream, if requested.
    pub(crate) fn beautify_code(&self, code: TokenStream) -> Result<String, WebCodeGenError> {
        if !self.beautify {
            return Ok(code.to_string());
        }

        let code_string = code.to_string();

        // Parse the generated code string into a syn::Item
        let syntax_tree: File = syn::parse_str(&code_string)?;

        // Use prettyplease to format the syntax tree
        let formatted_code = unparse(&syntax_tree);

        Ok(formatted_code)
    }

    /// Returns the output directory.
    ///
    /// # Errors
    ///
    /// * Raises a `GenerationDirectoryNotProvided` when the output directory is not provided.
    ///
    pub fn get_output_directory(&self) -> Result<&Path, CodeGenerationError> {
        self.output_directory.ok_or(CodeGenerationError::GenerationDirectoryNotProvided)
    }

    /// Writes all the tables syn version to a file.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    /// * `table_catalog` - The name of the table catalog.
    /// * `table_schema` - The name of the table schema.
    ///
    /// # Errors
    ///
    /// * Returns an error if the output path is not provided.
    /// * Returns an error if the tables cannot be loaded.
    /// * Returns an error if the tables cannot be converted to schema.
    /// * Returns an error if the tables cannot be converted to syn.
    /// * Returns an error if the user defined types cannot be loaded.
    /// * Returns an error if the user defined types cannot be converted to syn.
    /// * Returns an error if the generated code cannot be written to the output
    ///   file.
    pub fn generate(
        &self,
        conn: &mut PgConnection,
        table_catalog: &str,
        table_schema: Option<&str>,
    ) -> Result<(), WebCodeGenError> {
        let mut tables = Table::load_all(conn, table_catalog, table_schema)?
            .into_iter()
            .filter(|table| !(table.is_temporary() || table.is_view()))
            .filter(|table| !self.tables_deny_list.contains(&table))
            .collect::<Vec<Table>>();

        tables.sort_unstable();

        let codegen_directory = self.get_output_directory()?.join("codegen");
        std::fs::create_dir_all(&codegen_directory)?;
        let codegen_module = codegen_directory.with_extension("rs");

        self.generate_diesel_code(
            codegen_directory.as_path().join("diesel_codegen").as_path(),
            &tables,
            conn,
        )?;

        let codegen_module_impl = self.beautify_code(quote::quote! {
            pub mod diesel_codegen;
        })?;

        std::fs::write(&codegen_module, codegen_module_impl)?;

        Ok(())
    }
}
