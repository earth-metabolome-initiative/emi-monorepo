//! Submodule defining the trait builder implementation for a table and all
//! its extensions.

use quote::quote;

mod generate_setter_method;
mod mermaid_illustration;
use mermaid_illustration::columns_to_mermaid_illustration;

use crate::{
    Column, ColumnSameAsNetwork, PgExtension, Table, TableExtensionNetwork, TableLike,
    errors::WebCodeGenError,
};

impl Table {
    /// Returns the closest "same as" column in the current table or one of its
    /// ancestors, if any.
    ///
    /// # Arguments
    ///
    /// * `ancestor_column`: The column for which we want to find the closest
    ///   "same as" column.
    /// * `conn`: The PostgreSQL connection to use to retrieve information about
    ///   the table.
    ///
    /// # Errors
    ///
    /// * If the connection to the database fails.
    fn closest_same_as_column_in_table_or_ancestor(
        &self,
        ancestor_column: &crate::Column,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<(usize, crate::Column)>, WebCodeGenError> {
        if self.has_column(ancestor_column) {
            return Ok(Some((1, ancestor_column.clone())));
        }

        for column in self.insertable_columns(conn, false)? {
            if column.is_ancestrally_same_as(ancestor_column, conn)? {
                return Ok(Some((1, column)));
            }
        }

        let mut candidates = Vec::new();

        for ancestor in self.extension_tables(conn)? {
            if let Some((distance, matching_column)) =
                ancestor.closest_same_as_column_in_table_or_ancestor(ancestor_column, conn)?
            {
                candidates.push((distance + 1, matching_column));
            }
        }

        candidates.sort_by_key(|(distance, _)| *distance);
        Ok(candidates.first().cloned())
    }

    /// Dispatches the body of the setter method for a given insertable column.
    ///
    /// # Arguments
    ///
    /// * `extension_network`: The network of table extensions to which the
    ///   current table belongs.
    /// * `same_as_network`: The network of "same as" columns to which the
    ///   current table belongs.
    /// * `conn`: The PostgreSQL connection to use to retrieve information about
    ///   the table.
    /// * `insertable_column`: The insertable column for which the setter method
    ///   body must be generated.
    ///
    /// # Errors
    ///
    /// * If the method body cannot be generated.
    fn dispatch_setter_method_body(
        &self,
        conn: &mut diesel::PgConnection,
        insertable_column: &crate::Column,
        extension_network: &TableExtensionNetwork,
        same_as_network: &ColumnSameAsNetwork,
        check_constraints_extensions: &[&PgExtension],
    ) -> Result<(bool, bool, bool, Vec<Column>, proc_macro2::TokenStream), WebCodeGenError> {
        // If the provided column is part of the current table, we generate
        // the standard setter method body.
        if self.has_column(insertable_column) {
            return self
                .generate_setter_method(
                    insertable_column,
                    extension_network,
                    same_as_network,
                    check_constraints_extensions,
                    conn,
                )
                .map(|(requires_attribute_mutability, body, involved_columns)| {
                    (false, true, requires_attribute_mutability, involved_columns, body)
                });
        }

        let snake_case_ident = insertable_column.snake_case_ident()?;

        // We find the column closest to the current table (potentially in the current
        // table itself) that is "same as" the provided column, if any.

        if let Some((_, closest_same_as_column)) =
            self.closest_same_as_column_in_table_or_ancestor(insertable_column, conn)?
        {
            let setter_method = closest_same_as_column.getter_ident()?;
            let camel_case_ident = closest_same_as_column.camel_case_ident()?;
            let table = closest_same_as_column.table(conn)?;
            let setter_trait = if self == &table {
                let trait_ident = self.builder_trait_ident()?;
                quote! { #trait_ident }
            } else {
                let trait_path = table.builder_trait_ty()?;
                quote! { #trait_path }
            };

            if &closest_same_as_column == insertable_column {
                let foreign_key = &extension_network
                    .extension_foreign_keys_path(self, insertable_column, conn)
                    .expect(&format!(
                        "There should exist a foreign key path from table {self} to column {insertable_column}",
                    ))[0];
                let foreign_key_ident = foreign_key.constraint_ident(conn)?;
                let foreign_table = foreign_key.foreign_table(conn)?.unwrap();
                let foreign_table_ident = foreign_table.struct_ident()?;
                Ok((
                    true,
                    true,
                    false,
                    vec![insertable_column.clone()],
                    quote! {
                        self.#foreign_key_ident = <#foreign_table_ident as #setter_trait>::#setter_method(
                            self.#foreign_key_ident, #snake_case_ident
                        ).map_err(|e| e.into_field_name(|attribute| Self::Attributes::Extension(attribute.into())))?;
                        Ok(self)
                    },
                ))
            } else {
                Ok((
                    true,
                    false,
                    false,
                    vec![insertable_column.clone(), closest_same_as_column.clone()],
                    if insertable_column.is_nullable() && !closest_same_as_column.is_nullable() {
                        quote! {
                            <Self as #setter_trait>::#setter_method(
                                self,
                                #snake_case_ident.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                                    Self::Attributes::#camel_case_ident,
                                ))?
                            )
                        }
                    } else {
                        quote! {
                            <Self as #setter_trait>::#setter_method(self, #snake_case_ident)
                        }
                    },
                ))
            }
        } else {
            panic!(
                "The column {insertable_column} is not part of the table {self}, and there exists no ancestral column which is in a same-as relationship with it"
            );
        }
    }

    /// Implements for the builder associated to the current table
    /// the trait associated with the provided ancestral table.
    ///
    /// # Arguments
    ///
    /// * `extension_network`: The network of table extensions to which the
    ///   current table belongs.
    /// * `same_as_network`: The network of "same as" columns to which the
    ///   current table belongs.
    /// * `conn`: The PostgreSQL connection to use to retrieve information about
    ///   the table.
    /// * `ancestral_table`: The ancestral table for which the trait
    ///   implementation must be generated.
    ///
    /// # Errors
    ///
    /// * If the trait implementation cannot be generated.
    fn generate_builder_trait_impl_for_ancestral_table(
        &self,
        extension_network: &TableExtensionNetwork,
        same_as_network: &ColumnSameAsNetwork,
        conn: &mut diesel::PgConnection,
        ancestral_table: &Table,
        check_constraints_extensions: &[&PgExtension],
    ) -> Result<proc_macro2::TokenStream, WebCodeGenError> {
        let trait_path = if self == ancestral_table {
            let trait_ident = self.builder_trait_ident()?;
            quote! { #trait_ident }
        } else {
            let trait_path = ancestral_table.builder_trait_ty()?;
            quote! { #trait_path }
        };
        let builder_ident = self.insertable_builder_ident()?;
        let extension_tables = extension_network.extension_tables(self);
        let left_generics = extension_tables
            .iter()
            .map(|extension_table| {
                let generic_ident = extension_table.struct_ident()?;
                let trait_ident = extension_table.builder_trait_ty()?;
                let generic_attributes = extension_table.insertable_enum_ty()?;
                Ok(quote! { #generic_ident: #trait_ident<Attributes = #generic_attributes> })
            })
            .collect::<Result<Vec<_>, WebCodeGenError>>()?;
        let right_generics = extension_tables
            .iter()
            .map(|extension_table| extension_table.struct_ident())
            .collect::<Result<Vec<_>, WebCodeGenError>>()?;

        let maybe_left_generics = if left_generics.is_empty() {
            quote! {}
        } else {
            quote! { < #(#left_generics),* > }
        };
        let maybe_right_generics = if right_generics.is_empty() {
            quote! {}
        } else {
            quote! { < #(#right_generics),* > }
        };

        let method_impls = ancestral_table
            .insertable_columns(conn, false)?
            .iter()
            .map(|insertable_column| {
                let setter_method = insertable_column.getter_ident()?;
                let column_snake_case_ident = insertable_column.snake_case_ident()?;
                let maybe_generics = self.builder_trait_generics(&insertable_column, conn)?;
                let argument_type = self.builder_trait_argument_type(&insertable_column, conn)?;
                let maybe_where_constraints =
                    self.builder_trait_where_constraints(&insertable_column, conn)?;

                let (
                    include_inlining,
                    mutable_self,
                    mutable_attribute,
                    relevant_columns,
                    method_body,
                ) = self.dispatch_setter_method_body(
                    conn,
                    insertable_column,
                    extension_network,
                    same_as_network,
                    check_constraints_extensions,
                )?;

                let maybe_inline = include_inlining.then(|| quote! { #[inline] });
                let maybe_mut_self = mutable_self.then(|| quote! { mut });
                let maybe_mut_attribute = mutable_attribute.then(|| quote! { mut });
                let mut documentation =
                    vec![format!("Sets the value of the {insertable_column} column.",)];

                if relevant_columns.len() > 1 {
                    let mermaid_illustraion = columns_to_mermaid_illustration(
                        &relevant_columns,
                        &insertable_column,
                        conn,
                    )?;
                    let mermaid_illustraion_string = mermaid_illustraion.to_string();

                    documentation.extend(vec![
                        String::new(),
                        "# Implementation notes".to_string(),
                        "This method also set the values of other columns, due to".to_string(),
                        "same-as relationships or inferred values.".to_string(),
                        String::new(),
                        "## Mermaid illustration".to_string(),
                        String::new(),
                        "```mermaid".to_string(),
                    ]);
                    documentation
                        .extend(mermaid_illustraion_string.lines().map(|line| line.to_string()));
                    documentation.push("```".to_string());
                }

                Ok(quote! {
                    #maybe_inline
                    #(#[doc = #documentation])*
                    fn #setter_method #maybe_generics(
                        #maybe_mut_self self,
                        #maybe_mut_attribute #column_snake_case_ident: #argument_type
                    ) -> Result<
                        Self,
                        web_common_traits::database::InsertError<Self::Attributes>
                    > #maybe_where_constraints
                        {
                            #method_body
                        }
                })
            })
            .collect::<Result<Vec<_>, WebCodeGenError>>()?;

        let maybe_attributes = if ancestral_table.extension_tables(conn)?.is_empty() {
            let attributes = self.insertable_enum_ty()?;
            Some(quote! {
                type Attributes = #attributes;
            })
        } else {
            None
        };

        Ok(quote! {
            impl #maybe_left_generics #trait_path for #builder_ident #maybe_right_generics {
                #maybe_attributes

                #(#method_impls)*
            }
        })
    }

    /// Implements for the builder associated to the current table
    /// the trait associated with all ancestral tables.
    ///
    /// # Arguments
    ///
    /// * `extension_network`: The network of table extensions to which the
    ///   current table belongs.
    /// * `same_as_network`: The network of "same as" columns to which the
    ///   current table belongs.
    /// * `conn`: The PostgreSQL connection to use to retrieve information about
    ///   the table.
    /// * `check_constraints_extensions`: The PostgreSQL extensions that may be
    ///  required to implement the check constraints for the current table.
    ///
    /// # Errors
    ///
    /// * If the trait implementation cannot be generated.
    pub(super) fn generate_builder_trait_impl_for_ancestral_tables(
        &self,
        extension_network: &TableExtensionNetwork,
        same_as_network: &ColumnSameAsNetwork,
        conn: &mut diesel::PgConnection,
        check_constraints_extensions: &[&PgExtension],
    ) -> Result<Vec<proc_macro2::TokenStream>, WebCodeGenError> {
        let mut impls = vec![self.generate_builder_trait_impl_for_ancestral_table(
            extension_network,
            same_as_network,
            conn,
            self,
            check_constraints_extensions,
        )?];

        for ancestor in self.ancestral_extension_tables(conn)? {
            impls.push(self.generate_builder_trait_impl_for_ancestral_table(
                extension_network,
                same_as_network,
                conn,
                &ancestor,
                check_constraints_extensions,
            )?);
        }

        Ok(impls)
    }
}
