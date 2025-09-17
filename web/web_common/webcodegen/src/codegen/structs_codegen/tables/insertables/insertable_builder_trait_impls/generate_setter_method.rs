use std::collections::{HashMap, HashSet};

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{Column, PgExtension, Table, errors::WebCodeGenError};

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
        check_constraints_extensions: &[&PgExtension],
        extension_table_traits: &mut HashMap<Table, HashSet<Table>>,
        conn: &mut PgConnection,
    ) -> Result<(bool, TokenStream, Vec<Column>), WebCodeGenError> {
        assert!(self.has_column(column), "The column must belong to the current table");

        // We assert that the provided column is not part of the primary key,
        // as the primary key setter methods are generated separately.

        let insertable_enum = self.attributes_enum_ident()?;
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

        let (mut requires_attribute_mutability, same_as_assignments, same_as_columns) =
            self.generate_same_as_assignments(column, extension_table_traits, conn)?;

        involved_columns.extend(same_as_columns);
        involved_columns.sort_unstable();
        involved_columns.dedup();

        let column_camel_case_ident = column.camel_case_ident()?;
        let column_snake_case_ident = column.snake_case_ident()?;
        let mut maybe_column_preprocessing = None;

        let column_assignment = if let Some((
            partial_builder_kind,
            _,
            _partial_builder_foreign_key,
        )) = column.requires_partial_builder(conn)?
        {
            if partial_builder_kind.is_discretional() {
                let maybe_mut = requires_attribute_mutability.then(|| quote! { mut });
                maybe_column_preprocessing = Some(quote! {
                    let #maybe_mut #column_snake_case_ident = #column_snake_case_ident.into();
                });
                requires_attribute_mutability = false;
            }
            quote! {
                self.#column_snake_case_ident = #column_snake_case_ident;
            }
        } else {
            maybe_column_preprocessing =
                Some(if column.is_foreign_primary_key(conn)? || column.is_primary_key(conn)? {
                    let column_acronym = column.acronym_ident()?;
                    if column.is_nullable() {
                        quote! {
                            let #column_snake_case_ident = <
                                #column_acronym as web_common_traits::database::MaybePrimaryKeyLike
                            >::maybe_primary_key(&#column_snake_case_ident);
                        }
                    } else {
                        quote! {
                            let #column_snake_case_ident = <
                                #column_acronym as web_common_traits::database::PrimaryKeyLike
                            >::primary_key(&#column_snake_case_ident);
                        }
                    }
                } else {
                    quote! {
                        let #column_snake_case_ident = #column_snake_case_ident
                            .try_into()
                            .map_err(|err| {
                                validation_errors::SingleFieldError::from(err)
                                    .rename_field(#insertable_enum::#column_camel_case_ident)
                            })?;
                    }
                });

            if column.is_nullable() {
                quote! {
                    self.#column_snake_case_ident = #column_snake_case_ident;
                }
            } else {
                quote! {
                    self.#column_snake_case_ident = Some(#column_snake_case_ident);
                }
            }
        };

        Ok((
            requires_attribute_mutability,
            quote! {
                #maybe_column_preprocessing
                #updated_by_exception
                #(#same_as_assignments)*
                #(#check_constraints)*
                #column_assignment;
                Ok(self)
            },
            involved_columns,
        ))
    }
}
