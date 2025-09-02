//! Submodule defining the trait for the variant builder associated
//! with a table.

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{
    Column, Table, TableLike,
    codegen::{
        CODEGEN_DIRECTORY, CODEGEN_INSERTABLES_PATH, CODEGEN_STRUCTS_MODULE, CODEGEN_TABLES_PATH,
    },
    errors::WebCodeGenError,
};

impl Table {
    /// Returns the name of the trait for the variant builder.
    ///
    /// # Errors
    ///
    /// * If the name of the variant builder cannot be retrieved.
    pub fn builder_trait_name(&self) -> Result<String, WebCodeGenError> {
        Ok(format!("{}Buildable", self.struct_name()?))
    }

    /// Returns the [`Ident`](syn::Ident) for the variant builder
    /// trait.
    ///
    /// # Errors
    ///
    /// * If the name of the variant builder cannot be retrieved.
    pub fn builder_trait_ident(&self) -> Result<Ident, WebCodeGenError> {
        Ok(Ident::new(&self.builder_trait_name()?, proc_macro2::Span::call_site()))
    }

    /// Returns the [`Type`](syn::Type) for the variant builder trait.
    ///
    /// # Errors
    ///
    /// * If the name of the variant builder cannot be retrieved.
    pub fn builder_trait_ty(&self) -> Result<syn::Type, WebCodeGenError> {
        Ok(syn::parse_str(&format!(
            "crate::{CODEGEN_DIRECTORY}::{CODEGEN_STRUCTS_MODULE}::{CODEGEN_TABLES_PATH}::{CODEGEN_INSERTABLES_PATH}::{}",
            self.builder_trait_name()?
        ))?)
    }

    pub(super) fn builder_trait_generics(
        &self,
        insertable_column: &Column,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<TokenStream>, WebCodeGenError> {
        if insertable_column.is_foreign_primary_key(conn)?
            || insertable_column.is_part_of_primary_key(conn)?
        {
            return Ok(None);
        }

        let column_acronym = insertable_column.acronym_ident()?;

        Ok(Some(quote! {
            <#column_acronym>
        }))
    }

    pub(super) fn builder_trait_argument_type(
        &self,
        insertable_column: &Column,
        conn: &mut diesel::PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        if let Some(partial_builder_foreign_key) =
            insertable_column.requires_partial_builder(conn)?
        {
            let foreign_table = partial_builder_foreign_key
                .foreign_table(conn)?
                .expect("The foreign table should be present");
            let foreign_builder_type = foreign_table.insertable_builder_ty()?;
            Ok(quote! {#foreign_builder_type})
        } else if insertable_column.is_foreign_primary_key(conn)?
            || insertable_column.is_part_of_primary_key(conn)?
        {
            let rust_type = insertable_column.rust_data_type(conn)?;
            Ok(quote! { #rust_type })
        } else {
            let column_acronym = insertable_column.acronym_ident()?;
            Ok(quote! { #column_acronym })
        }
    }

    pub(super) fn builder_trait_where_constraints(
        &self,
        insertable_column: &Column,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<TokenStream>, WebCodeGenError> {
        if insertable_column.is_foreign_primary_key(conn)?
            || insertable_column.is_part_of_primary_key(conn)?
        {
            return Ok(None);
        }

        let column_acronym = insertable_column.acronym_ident()?;
        let argument_type = insertable_column.rust_data_type(conn)?;

        Ok(Some(quote! {
            where
                #column_acronym: TryInto<#argument_type>,
                validation_errors::SingleFieldError: From<<#column_acronym as TryInto<#argument_type>>::Error>
        }))
    }

    /// Returns the trait definition for the builder associated to the current
    /// table.
    ///
    /// # Arguments
    ///
    /// * `conn`: The PostgreSQL connection to use to retrieve information about
    ///   the table.
    ///
    /// # Errors
    ///
    /// * If the trait definition cannot be generated.
    /// * If the name of the variant builder cannot be retrieved.
    pub(super) fn generate_builder_trait(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<proc_macro2::TokenStream, WebCodeGenError> {
        let trait_ident = self.builder_trait_ident()?;
        let mut methods = Vec::new();

        for insertable_column in self.insertable_columns(conn, false)? {
            let setter_method = insertable_column.getter_ident()?;
            let column_snake_case_ident = insertable_column.snake_case_ident()?;
            let column_snake_case_name = insertable_column.snake_case_name()?;
            let column_str_type = insertable_column.str_rust_data_type(conn)?;
            let maybe_generics = self.builder_trait_generics(&insertable_column, conn)?;
            let argument_type = self.builder_trait_argument_type(&insertable_column, conn)?;
            let maybe_where_constraints =
                self.builder_trait_where_constraints(&insertable_column, conn)?;

            if insertable_column.is_most_concrete_table(conn)? {
                continue;
            }

            let method_documentations =
                vec![
					format!("Sets the value of the {insertable_column} column."),
					"".to_owned(),
					format!("# Arguments"),
					format!("* `{column_snake_case_name}`: The value to set for the {insertable_column} column."),
					"".to_owned(),
					"# Implementation details".to_owned(),
					concat!(
						"This method accepts a reference to a generic value which can be converted to the required type for the column. ",
						"This allows passing values of different types, as long as they can be converted to the required type using the `TryFrom` trait. ",
						"The method, additionally, employs same-as and inferred same-as rules to ensure that the schema-defined ancestral tables and ",
						"associated table values associated to the current column (if any) are also set appropriately."
					).to_owned(),
					"".to_owned(),
					format!("# Errors"),
					format!("* If the provided value cannot be converted to the required type `{column_str_type}`."),
					format!("* If the provided value does not pass schema-defined validation."),
				];

            methods.push(quote! {
                #(#[doc = #method_documentations])*
                fn #setter_method #maybe_generics(
                    self,
                    #column_snake_case_ident: #argument_type
                ) -> Result<
                    Self,
                    web_common_traits::database::InsertError<Self::Attributes>
                > #maybe_where_constraints;
            })
        }

        let trait_documentation = format!(
            "Trait defining setters for attributes of an instance of `{}` or descendant tables.",
            self.struct_name()?
        );

        Ok(quote::quote! {
            #[doc = #trait_documentation]
            pub trait #trait_ident: Sized {
                /// Attributes required to build the insertable.
                type Attributes;

                #(#methods)*
            }
        })
    }
}
