//! Submodule defining the trait builder implementation for a table and all
//! its extensions.

use std::collections::{HashMap, HashSet};

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    Column, Table, TableLike, errors::WebCodeGenError, table_metadata::PartialBuilderKind,
};

impl Table {
    /// Returns the assigment methods associated with the same-as and inferred
    /// same-as relationships for the provided column in the context of the
    /// provided table.
    ///
    /// Furthermore, it returns the list of ancestral columns that are assigned
    /// in the same-as relationships.
    ///
    /// # Arguments
    ///
    /// * `table`: The table for which we are currently generating the
    ///   insertable builder.
    /// * `current_column`: The column for which we are current the insertable
    ///   setter method for the insertable builder.
    /// * `conn`: The database connection.
    ///
    /// # Implementation details
    ///
    /// There exist two types of columns which we need to handle:
    ///
    /// 1. Columns that are part of the ancestors' of the current table's
    ///    attributes.
    /// 2. Columns that are part of other tables' that require partial builders.
    pub(super) fn generate_same_as_assignments(
        &self,
        current_column: &Column,
        extension_network: &crate::TableExtensionNetwork,
        extension_table_traits: &mut HashMap<Table, HashSet<Table>>,
        conn: &mut PgConnection,
    ) -> Result<(bool, Vec<TokenStream>, Vec<Column>), WebCodeGenError> {
        if self.has_composite_primary_key(conn)? {
            return Ok((false, Vec::new(), Vec::new()));
        }
        assert!(self.has_column(current_column), "The column must belong to the current table");

        let current_column_ident = current_column.snake_case_ident()?;
        let current_column_camel_case_ident = current_column.camel_case_ident()?;
        let mut assignments = Vec::new();
        let mut requires_mutability = false;
        let mut involved_columns: Vec<Column> = Vec::new();
        let required_ancestor_columns: Vec<Column> =
            current_column.all_ancestral_same_as_columns(conn)?;
        involved_columns.extend(required_ancestor_columns.clone());
        involved_columns.extend(current_column.foreign_defined_columns(false, conn)?);

        // We iterate over the direct ancestor table, as the ancestors must fall
        // within two categories:
        //
        // 1. The ancestor table contains a column which is in a same-as relationship
        //    with the current column and potentially additional columns.
        // 2. The ancestor table does not contain any column which is in a same-as
        //    relationship with the current column, but forwards the methos to set the
        //    column from an ancestor which does.

        for required_ancestor_column in &required_ancestor_columns {
            let foreign_key = &extension_network
                .extension_foreign_keys_path(self, required_ancestor_column, conn)
                .expect(&format!(
                    "There should exist a foreign key path from table {self} to column {required_ancestor_column}",
                ))[0];

            let required_ancestor_table = required_ancestor_column.table(conn)?;
            let buildable_trait = required_ancestor_table.setter_trait_ty()?;
            let foreign_table = foreign_key.foreign_table(conn)?;
            let generic_ident = foreign_table.struct_ident()?;
            let foreign_key_ident = foreign_key.constraint_ident(conn)?;
            let column_setter = required_ancestor_column.getter_ident()?;

            // The current column is in a same-as relationship with a column
            // from a direct ancestor table.
            extension_table_traits
                .get_mut(&foreign_table)
                .unwrap()
                .insert(required_ancestor_table.as_ref().clone());
            assignments.push(quote! {
                self.#foreign_key_ident = <#generic_ident as #buildable_trait>::#column_setter(
                    self.#foreign_key_ident,
                    #current_column_ident
                ).map_err(|err| {
                    err.into_field_name(|attribute| Self::Attributes::Extension(attribute.into()))
                })?;
            });
        }

        if let Some((partial_builder_kind, _, foreign_key)) =
            current_column.requires_partial_builder(conn)?
        {
            let foreign_table = foreign_key.foreign_table(conn)?;

            for associated_same_as_foreign_key in
                self.associated_same_as_foreign_keys(false, conn)?
            {
                let local_columns = associated_same_as_foreign_key.columns(conn)?;

                if !local_columns.contains(current_column) || local_columns.len() == 1 {
                    continue;
                }

                if associated_same_as_foreign_key.foreign_table(conn)? != foreign_table {
                    continue;
                }

                let foreign_columns = associated_same_as_foreign_key.foreign_columns(conn)?;

                let (local_column, foreign_column) = local_columns
                    .iter()
                    .zip(foreign_columns.iter())
                    .find(|(local_column, _foreign_column)| local_column != &current_column)
                    .expect(&format!("The current column {current_column} must be part of the foreign key from ({}) to ({}).",
                        local_columns.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", "),
                        foreign_columns.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", "),
                    ));

                involved_columns.push(local_column.clone());
                involved_columns.push(foreign_column.clone());
                let local_column_ident = local_column.snake_case_ident()?;
                let local_column_camel_case_ident = local_column.camel_case_ident()?;
                let foreign_builder = foreign_table.insertable_builder_ty()?;
                let foreign_table_trait = foreign_table.setter_trait_ty()?;
                let foreign_column_ident = foreign_column.snake_case_ident()?;
                let foreign_column_setter_ident = foreign_column.getter_ident()?;

                requires_mutability = true;

                // We determine the names of the local and foreign identifiers.
                // When these columns have distinct names, we can directly use them,
                // otherwise we call the local one `local_...` and the foreign one
                // `foreign_...`.
                let (local_column_ident, foreign_column_ident) =
                    if local_column_ident != foreign_column_ident {
                        (local_column_ident, foreign_column_ident)
                    } else {
                        (
                            quote::format_ident!("local_{}", local_column_ident),
                            quote::format_ident!("foreign_{}", foreign_column_ident),
                        )
                    };

                assignments.push(match partial_builder_kind {
                    PartialBuilderKind::Mandatory => {
                        quote! {
                            if let (Some(#local_column_ident), Some(#foreign_column_ident)) = (self.#local_column_ident, #current_column_ident.#foreign_column_ident) {
                                if #local_column_ident != #foreign_column_ident {
                                    return Err(
                                        web_common_traits::database::InsertError::BuilderError(
                                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                                Self::Attributes::#local_column_camel_case_ident
                                            )
                                        )
                                    );
                                }
                            } else if let Some(#foreign_column_ident) = #current_column_ident.#foreign_column_ident {
                                self.#local_column_ident = Some(#foreign_column_ident);
                            } else if let Some(#local_column_ident) = self.#local_column_ident {
                                #current_column_ident = <#foreign_builder as #foreign_table_trait>::#foreign_column_setter_ident(
                                    #current_column_ident,
                                    #local_column_ident
                                ).map_err(|e| {
                                    e.into_field_name(|attribute| {
                                        Self::Attributes::#current_column_camel_case_ident(attribute)
                                    })
                                })?;
                            }
                        }
                    }
                    PartialBuilderKind::Discretional => {
                        quote! {
                            if let web_common_traits::database::IdOrBuilder::Builder(builder) = #current_column_ident {
                                #current_column_ident = if let (Some(#local_column_ident), Some(#foreign_column_ident)) = (self.#local_column_ident, builder.#foreign_column_ident) {
                                    if #local_column_ident != #foreign_column_ident {
                                        return Err(
                                            web_common_traits::database::InsertError::BuilderError(
                                                web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                                    Self::Attributes::#local_column_camel_case_ident
                                                )
                                            )
                                        );
                                    }
                                    builder.into()
                                } else if let Some(#foreign_column_ident) = builder.#foreign_column_ident {
                                    self.#local_column_ident = Some(#foreign_column_ident);
                                    builder.into()
                                } else if let Some(#local_column_ident) = self.#local_column_ident {
                                    <#foreign_builder as #foreign_table_trait>::#foreign_column_setter_ident(
                                        builder,
                                        #local_column_ident
                                    ).map_err(|e| {
                                        e.into_field_name(|attribute| {
                                            Self::Attributes::#current_column_camel_case_ident(attribute)
                                        })
                                    })?.into()
                                } else {
                                    builder.into()
                                };
                            }
                        }
                    }
                });
            }
        } else {
            for (foreign_column, associated_same_as_constraint) in
                current_column.associated_same_as_columns(false, conn)?
            {
                let local_columns = associated_same_as_constraint.columns(conn)?;
                assert_eq!(
                    local_columns.len(),
                    2,
                    "Found a same-as constraint in {}.{} with {} columns: {}",
                    self.table_schema,
                    self.table_name,
                    local_columns.len(),
                    associated_same_as_constraint.to_sql(conn)?
                );
                let local_column = &local_columns[0];

                involved_columns.push(foreign_column.clone());
                involved_columns.push(local_column.clone());

                let foreign_table = associated_same_as_constraint.foreign_table(conn)?;
                let foreign_builder = foreign_table.insertable_builder_ty()?;
                let foreign_table_trait = foreign_table.setter_trait_ty()?;

                let local_column_ident = local_column.snake_case_ident()?;
                let local_column_camel_case_ident = local_column.camel_case_ident()?;

                let Some((partial_builder_kind, _, _)) =
                    local_column.requires_partial_builder(conn)?
                else {
                    unreachable!("The local column {local_column} must be a partial builder");
                };

                let foreign_column_setter_ident = foreign_column.getter_ident()?;

                assignments.push(match partial_builder_kind {
                    PartialBuilderKind::Mandatory => {
                        quote! {
                            self.#local_column_ident = <#foreign_builder as #foreign_table_trait>::#foreign_column_setter_ident(
                                self.#local_column_ident,
                                #current_column_ident
                            ).map_err(|e| {
                                e.into_field_name(|attribute| {
                                    Self::Attributes::#local_column_camel_case_ident(attribute)
                                })
                            })?;
                        }
                    }
                    PartialBuilderKind::Discretional => {
                        quote! {
                            if let web_common_traits::database::IdOrBuilder::Builder(#local_column_ident) = self.#local_column_ident {
                                self.#local_column_ident = <#foreign_builder as #foreign_table_trait>::#foreign_column_setter_ident(
                                    #local_column_ident,
                                    #current_column_ident
                                ).map_err(|e| {
                                    e.into_field_name(|attribute| {
                                        Self::Attributes::#local_column_camel_case_ident(attribute)
                                    })
                                })?.into();
                            }
                        }
                    }
                });
            }
        }
        Ok((requires_mutability, assignments, involved_columns))
    }
}
