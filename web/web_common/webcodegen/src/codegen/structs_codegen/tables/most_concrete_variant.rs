//! Submodule defining the structs supporting the [`MostConcreteVariant`] and
//! [`MostConcreteVariant`]-adjacent traits.

use std::path::Path;

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

mod most_concrete_variant_builder_enum_implementation;
mod most_concrete_variant_enum_implementation;
mod most_concrete_variant_insert_error_enum_implementation;

use crate::{
    Codegen, Table,
    codegen::{
        CODEGEN_DIRECTORY, CODEGEN_MOST_CONCRETE_VARIANTS_PATH, CODEGEN_STRUCTS_MODULE,
        CODEGEN_TABLES_PATH,
    },
    errors::WebCodeGenError,
    traits::TableLike,
};

impl Table {
    /// Returns the name for the variant DAG.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the name of the variant DAG cannot be retrieved.
    pub fn dag_name(&self, conn: &mut PgConnection) -> Result<Option<String>, WebCodeGenError> {
        let Some(most_concrete_table_column) = self.most_concrete_table_column(true, conn)? else {
            return Ok(None);
        };
        let table = most_concrete_table_column.table(conn)?;
        Ok(Some(format!("{}DAG", table.struct_name()?)))
    }

    /// Returns the name for the variant builder DAG.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the name of the variant DAG cannot be retrieved.
    pub fn builder_dag_name(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<String>, WebCodeGenError> {
        let Some(most_concrete_table_column) = self.most_concrete_table_column(true, conn)? else {
            return Ok(None);
        };
        let table = most_concrete_table_column.table(conn)?;
        Ok(Some(format!("{}BuilderDAG", table.struct_name()?)))
    }

    /// Returns the name for the variant insert error DAG.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the name of the variant DAG cannot be retrieved.
    pub fn insert_error_dag_name(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<String>, WebCodeGenError> {
        let Some(most_concrete_table_column) = self.most_concrete_table_column(true, conn)? else {
            return Ok(None);
        };
        let table = most_concrete_table_column.table(conn)?;
        Ok(Some(format!("{}InsertErrorDAG", table.struct_name()?)))
    }

    /// Returns the [`Ident`](syn::Ident) for the variant DAG.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the name of the variant DAG cannot be retrieved.
    pub fn dag_ident(&self, conn: &mut PgConnection) -> Result<Option<Ident>, WebCodeGenError> {
        let Some(name) = self.dag_name(conn)? else {
            return Ok(None);
        };
        Ok(Some(Ident::new(&name, proc_macro2::Span::call_site())))
    }

    /// Returns the [`Ident`](syn::Ident) for the variant builder DAG.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the name of the variant DAG cannot be retrieved.
    pub fn builder_dag_ident(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<Ident>, WebCodeGenError> {
        let Some(name) = self.builder_dag_name(conn)? else {
            return Ok(None);
        };
        Ok(Some(Ident::new(&name, proc_macro2::Span::call_site())))
    }

    /// Returns the [`Ident`](syn::Ident) for the variant insert error DAG.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the name of the variant DAG cannot be retrieved.
    pub fn insert_error_dag_ident(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<Ident>, WebCodeGenError> {
        let Some(name) = self.insert_error_dag_name(conn)? else {
            return Ok(None);
        };
        Ok(Some(Ident::new(&name, proc_macro2::Span::call_site())))
    }

    /// Returns the [`Type`](syn::Type) for the variant DAG.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the name of the variant DAG cannot be retrieved.
    pub fn dag_ty(&self, conn: &mut PgConnection) -> Result<Option<syn::Type>, WebCodeGenError> {
        let Some(name) = self.dag_name(conn)? else {
            return Ok(None);
        };
        Ok(Some(syn::parse_str(&format!(
            "crate::{CODEGEN_DIRECTORY}::{CODEGEN_STRUCTS_MODULE}::{CODEGEN_TABLES_PATH}::{CODEGEN_MOST_CONCRETE_VARIANTS_PATH}::{}",
            name
        ))?))
    }

    /// Returns the [`Type`](syn::Type) for the variant builder DAG.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the name of the variant DAG cannot be retrieved.
    pub fn builder_dag_ty(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<syn::Type>, WebCodeGenError> {
        let Some(name) = self.builder_dag_name(conn)? else {
            return Ok(None);
        };
        Ok(Some(syn::parse_str(&format!(
            "crate::{CODEGEN_DIRECTORY}::{CODEGEN_STRUCTS_MODULE}::{CODEGEN_TABLES_PATH}::{CODEGEN_MOST_CONCRETE_VARIANTS_PATH}::{}",
            name
        ))?))
    }
    /// Returns the [`Type`](syn::Type) for the variant insert error DAG.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the name of the variant DAG cannot be retrieved.
    pub fn insert_error_dag_ty(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<syn::Type>, WebCodeGenError> {
        let Some(name) = self.insert_error_dag_name(conn)? else {
            return Ok(None);
        };
        Ok(Some(syn::parse_str(&format!(
            "crate::{CODEGEN_DIRECTORY}::{CODEGEN_STRUCTS_MODULE}::{CODEGEN_TABLES_PATH}::{CODEGEN_MOST_CONCRETE_VARIANTS_PATH}::{}",
            name
        ))?))
    }
}

impl Codegen<'_> {
    /// Generate implementations of the structs able to implement the
    /// [`MostConcreteVariant`] and [`MostConcreteVariant`]-adjacent traits for
    /// the provided tables.
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
    pub(super) fn generate_most_concrete_variant_structs(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;

        let mut insertables_main_module = TokenStream::new();

        for table in tables {
            let Some(most_concrete_table_column) = table.most_concrete_table_column(false, conn)?
            else {
                continue;
            };

            let dag_ident: Ident = table.dag_ident(conn)?.unwrap();
            let builder_dag_ident: Ident = table.builder_dag_ident(conn)?.unwrap();
            let insert_error_dag_ident: Ident = table.insert_error_dag_ident(conn)?.unwrap();
            let enum_implementation = self.generate_most_concrete_variant_enum_implementation(
                table,
                &most_concrete_table_column,
                &dag_ident,
                conn,
            )?;

            let file_name = root.join(format!("{}.rs", table.snake_case_name()?));
            std::fs::write(
                &file_name,
                self.beautify_code(&quote! {
                    mod builder;
                    mod insert_error;
                    pub use insert_error::#insert_error_dag_ident;
                    pub use builder::#builder_dag_ident;

                    #enum_implementation
                }),
            )?;

            let builder_enum_implementation =
                self.generate_most_concrete_variant_builder_enum_implementation(table, conn)?;

            let table_directory = root.join(table.snake_case_name()?);
            std::fs::create_dir_all(&table_directory)?;
            let builder_file_name = table_directory.join("builder.rs");
            std::fs::write(
                &builder_file_name,
                self.beautify_code(&quote! {
                    #builder_enum_implementation
                }),
            )?;

            let insert_error_enum_implementation =
                self.generate_most_concrete_variant_insert_error_enum_implementation(table, conn)?;

            let insert_error_file_name = table_directory.join("insert_error.rs");
            std::fs::write(
                &insert_error_file_name,
                self.beautify_code(&quote! {
                    #insert_error_enum_implementation
                }),
            )?;

            let table_identifier = table.snake_case_ident()?;
            insertables_main_module.extend(quote! {
                mod #table_identifier;
                pub use #table_identifier::{#dag_ident, #builder_dag_ident, #insert_error_dag_ident};
            });
        }

        let insertables_file = root.with_extension("rs");
        std::fs::write(&insertables_file, self.beautify_code(&insertables_main_module))?;

        Ok(())
    }
}
