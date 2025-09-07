//! Submodule defining the `TableLike` trait.

use syn::{Ident, Type};

use crate::{
    Table,
    codegen::{CODEGEN_DIESEL_MODULE, CODEGEN_DIRECTORY, CODEGEN_TABLES_PATH},
    errors::WebCodeGenError,
    utils::RESERVED_RUST_WORDS,
};

/// A trait representing types that can be treated as tables.
pub trait TableLike: AsRef<Table> {
    /// Returns the sanitized snake case name of the table.
    ///
    /// # Errors
    ///
    /// * If the snake case name cannot be generated.
    ///
    /// # Returns
    ///
    /// A string representing the sanitized snake case name of the table.
    fn snake_case_name(&self) -> Result<String, WebCodeGenError> {
        crate::utils::snake_case_name(&self.as_ref().table_name)
    }

    /// Returns whether the table has a sanitized snake case name.
    ///
    /// # Errors
    ///
    /// * If the snake case name cannot be generated.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the table has a sanitized snake case name.
    ///
    /// # Errors
    ///
    /// * If the snake case name cannot be generated.
    fn has_snake_case_name(&self) -> Result<bool, WebCodeGenError> {
        let snake_case_name = self.snake_case_name()?;
        Ok(snake_case_name == self.as_ref().table_name)
    }

    /// Returns the sanitized snake case syn Ident of the table.
    ///
    /// # Errors
    ///
    /// * If the snake case name cannot be generated.
    ///
    /// # Returns
    ///
    /// A `Ident` representing the sanitized snake case name of the table.
    ///
    /// # Errors
    ///
    /// * If the snake case name cannot be generated.
    fn snake_case_ident(&self) -> Result<Ident, WebCodeGenError> {
        let snake_case_name = self.snake_case_name()?;
        if RESERVED_RUST_WORDS.contains(&snake_case_name.as_str()) {
            Ok(Ident::new_raw(&snake_case_name, proc_macro2::Span::call_site()))
        } else {
            Ok(Ident::new(&snake_case_name, proc_macro2::Span::call_site()))
        }
    }

    /// Returns the name of the struct converted from the table name.
    ///
    /// # Errors
    ///
    /// * If the camel case name cannot be generated.
    ///
    /// # Returns
    ///
    /// A string representing the name of the struct converted from the table
    /// name.
    fn struct_name(&self) -> Result<String, WebCodeGenError> {
        crate::utils::struct_name(&self.as_ref().table_name)
    }

    /// Returns the Rust Ident of the struct converted from the table name.
    ///
    /// # Errors
    ///
    /// * If the camel case name cannot be generated.
    fn struct_ident(&self) -> Result<Ident, WebCodeGenError> {
        let struct_name = self.struct_name()?;
        if RESERVED_RUST_WORDS.contains(&struct_name.as_str()) {
            Ok(Ident::new_raw(&struct_name, proc_macro2::Span::call_site()))
        } else {
            Ok(Ident::new(&struct_name, proc_macro2::Span::call_site()))
        }
    }

    /// Returns a the path to the diesel table module.
    ///
    /// # Returns
    ///
    /// A `syn::Type` representing the path to the diesel table module.
    ///
    /// # Errors
    ///
    /// * If the snake case name cannot be generated.
    fn import_diesel_path(&self) -> Result<syn::Type, WebCodeGenError> {
        let table_name = self.snake_case_name()?;
        Ok(syn::parse_str::<Type>(&format!(
            "crate::{CODEGEN_DIRECTORY}::{CODEGEN_DIESEL_MODULE}::{CODEGEN_TABLES_PATH}::{table_name}::{table_name}",
        ))?)
    }

    /// Returns a the path to the table struct.
    ///
    /// # Returns
    ///
    /// A `syn::Type` representing the path to the table struct.
    ///
    /// # Errors
    ///
    /// * If the snake case name cannot be generated.
    fn import_struct_path(&self) -> Result<syn::Type, WebCodeGenError> {
        Ok(syn::parse_str::<Type>(&format!("crate::{}", self.struct_name()?))?)
    }
}

impl<T> TableLike for T where T: AsRef<Table> {}
