//! Submodule defining the structs supporting the [`Insertable`] and
//! [`Insertable`]-adjacent traits.

use std::path::Path;

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{
    Codegen, Column, Table,
    codegen::{
        CODEGEN_DIRECTORY, CODEGEN_INSERTABLES_PATH, CODEGEN_STRUCTS_MODULE, CODEGEN_TABLES_PATH,
    },
    errors::WebCodeGenError,
    traits::TableLike,
};

mod insertable_builder_definition;
mod insertable_builder_set_primary_key;
mod insertable_builder_trait;
mod insertable_builder_trait_impls;
mod insertable_builder_try_insert_generic;
mod insertable_enum;
mod insertable_variant;

impl Table {
    /// Returns the name for the insertable variant builder.
    ///
    /// # Errors
    ///
    /// * If the name of the insertable variant builder cannot be retrieved.
    pub fn insertable_builder_name(&self) -> Result<String, WebCodeGenError> {
        Ok(format!("{}Builder", self.insertable_variant_name()?))
    }

    /// Returns the [`Ident`](syn::Ident) for the insertable variant builder.
    ///
    /// # Errors
    ///
    /// * If the name of the insertable variant builder cannot be retrieved.
    pub fn insertable_builder_ident(&self) -> Result<Ident, WebCodeGenError> {
        Ok(Ident::new(&self.insertable_builder_name()?, proc_macro2::Span::call_site()))
    }

    /// Returns the [`Type`](syn::Type) for the insertable variant builder.
    ///
    /// # Errors
    ///
    /// * If the name of the insertable variant builder cannot be retrieved.
    pub fn insertable_builder_ty(&self) -> Result<syn::Type, WebCodeGenError> {
        Ok(syn::parse_str(&format!(
            "crate::{CODEGEN_DIRECTORY}::{CODEGEN_STRUCTS_MODULE}::{CODEGEN_TABLES_PATH}::{CODEGEN_INSERTABLES_PATH}::{}",
            self.insertable_builder_name()?
        ))?)
    }

    /// Returns the list of insertable columns for the table.
    ///
    /// # Implementation details
    ///
    /// A column is considered an insertable column if:
    ///
    /// - it is not automatically generated
    /// - it is not a primary key column that is part of an extension, hence
    ///   defined when the extension is inserted.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    /// * `include_extension_columns` - A boolean indicating whether to include
    ///   extension columns in the result.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    pub(crate) fn insertable_columns(
        &self,
        conn: &mut PgConnection,
        include_extension_columns: bool,
    ) -> Result<Vec<Column>, WebCodeGenError> {
        let mut insertable_columns = Vec::new();

        for column in self.columns(conn)? {
            if column.is_always_automatically_generated()
                || !include_extension_columns
                    && column.is_part_of_extension_primary_key(conn)?.is_some()
            {
                continue;
            }
            insertable_columns.push(column);
        }

        Ok(insertable_columns)
    }
}

impl Codegen<'_> {
    /// Generate implementations of the structs able to implement the
    /// [`Insertable`] and [`Insertable`]-adjacent traits for the provided
    /// tables.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    /// * If the file system fails.
    pub(super) fn generate_insertable_structs(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;

        let mut insertables_main_module = TokenStream::new();

        for table in tables {
            let insertable_enum = table.insertable_enum_ident()?;
            let maybe_insertable_extension_enum = if table.extension_tables(conn)?.is_empty() {
                None
            } else {
                Some(table.insertable_extension_enum_ident()?)
            };
            let insertable_variant_ident = table.insertable_variant_ident()?;
            let insertable_builder_ident = table.insertable_builder_ident()?;
            let buildable_trait_ident = table.setter_trait_ident()?;
            let enum_implementation = table.insertable_enum_definition(conn)?;
            let insertable_variant_definition = table.insertable_variant_definition(conn)?;

            // When the table associated with the struct we are generating is not an
            // extension, we can implement the `TryFrom` trait to convert the insertable
            // builder into the insertable variant, which we will then be able to directly
            // insert into the database.
            let builder_definition = self.generate_insertable_builder_definition(table, conn)?;
            let maybe_set_primary_key_impl =
                self.generate_insertable_builder_set_primary_key(table, conn)?;
            let try_insert_generic_impl: TokenStream =
                self.generate_insertable_builder_try_insert_generic_implementation(table, conn)?;

            let ifv_file = root.join(format!("{}.rs", table.snake_case_name()?));
            std::fs::write(
                &ifv_file,
                self.beautify_code(&quote! {
                    #enum_implementation
                    #insertable_variant_definition
                    #builder_definition
                    #maybe_set_primary_key_impl

                    #try_insert_generic_impl
                })?,
            )?;

            let table_identifier = table.snake_case_ident()?;
            insertables_main_module.extend(quote! {
                mod #table_identifier;
                pub use #table_identifier::{#insertable_variant_ident, #insertable_builder_ident, #buildable_trait_ident, #insertable_enum, #maybe_insertable_extension_enum};
            });
        }

        let insertables_file = root.with_extension("rs");
        std::fs::write(&insertables_file, self.beautify_code(&insertables_main_module)?)?;

        Ok(())
    }
}
