use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    Column, ColumnSameAsNetwork, PgExtension, Table, TableExtensionNetwork, errors::WebCodeGenError,
};

mod generate_method_check_constraints;
mod generate_same_as_assignments;

impl Table {
    /// Generates the implementation of the setter method for the provided
    /// column for the builder of the provided table.
    ///
    /// # Arguments
    ///
    /// * `table` - The table for which to generate the setter method.
    /// * `columns` - The columns (potentially foreign) for which to generate
    ///   the setter method.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    ///
    /// # Panics
    ///
    /// * If the requested method to generate is associated to columns which are
    ///   all in same-as relationship with columns within the current table. In
    ///   such case, the method would be redoundant and potentially (if the
    ///   columns have the same name) conflicting.
    pub(super) fn generate_setter_method(
        &self,
        column: &Column,
        extension_network: &TableExtensionNetwork,
        same_as_network: &ColumnSameAsNetwork,
        check_constraints_extensions: &[&PgExtension],
        conn: &mut PgConnection,
    ) -> Result<(bool, TokenStream, Vec<Column>), WebCodeGenError> {
        assert!(self.has_column(column), "The column must belong to the current table");

        // We assert that the provided column is not part of the primary key,
        // as the primary key setter methods are generated separately.

        let insertable_enum = self.insertable_enum_ident()?;
        let mut involved_columns = vec![column.clone()];
        let check_constraints: Vec<TokenStream> =
            self.generate_method_check_constraints(&[column], check_constraints_extensions, conn)?;

        assert_eq!(
            check_constraints.len(),
            check_constraints
                .iter()
                .map(|ts| ts.to_string())
                .collect::<std::collections::HashSet<_>>()
                .len(),
            "The check constraints should be unique"
        );

        // If the current table has both `created_by` and `updated_by`, and
        // we are currently assigning the `created_by` column, we need to
        // assign the `updated_by` column as well.
        let updated_by_exception = if column.is_created_by(conn)? {
            if let Some(updated_by_column) = self.updated_by_column(false, conn)? {
                involved_columns.push(updated_by_column);
                Some(quote! {
                    self = self.updated_by(created_by)?;
                })
            } else {
                None
            }
        } else {
            None
        };

        let mut same_as_assignments = Vec::new();

        let (requires_attribute_mutability, this_same_as_assignments, same_as_columns) =
            self.generate_same_as_assignments(column, extension_network, same_as_network, conn)?;

        involved_columns.extend(same_as_columns);
        involved_columns.sort_unstable();
        involved_columns.dedup();

        same_as_assignments.extend(this_same_as_assignments);
        let column_camel_case_ident = column.camel_case_ident()?;
        let column_snake_case_ident = column.snake_case_ident()?;

        let (column_assignment, maybe_column_preprocessing) =
            if let Some(_partial_builder_foreign_key) = column.requires_partial_builder(conn)? {
                (
                    quote! {
                        #column_snake_case_ident;
                    },
                    None,
                )
            } else if column.is_nullable() {
                (
                    quote! {
                        #column_snake_case_ident;
                    },
                    Some(quote! {
                        let #column_snake_case_ident = #column_snake_case_ident
                            .try_into()
                            .map_err(|err| {
                                validation_errors::SingleFieldError::from(err)
                                    .rename_field(#insertable_enum::#column_camel_case_ident)
                            })?;
                    }),
                )
            } else {
                (
                    quote! {
                        Some(#column_snake_case_ident);
                    },
                    Some(quote! {
                        let #column_snake_case_ident = #column_snake_case_ident
                            .try_into()
                            .map_err(|err| {
                                validation_errors::SingleFieldError::from(err)
                                    .rename_field(#insertable_enum::#column_camel_case_ident)
                            })?;
                    }),
                )
            };

        Ok((
            requires_attribute_mutability,
            quote! {
                #updated_by_exception
                #maybe_column_preprocessing
                #(#same_as_assignments)*
                #(#check_constraints)*
                self.#column_snake_case_ident = #column_assignment;
                Ok(self)
            },
            involved_columns,
        ))
    }
}
