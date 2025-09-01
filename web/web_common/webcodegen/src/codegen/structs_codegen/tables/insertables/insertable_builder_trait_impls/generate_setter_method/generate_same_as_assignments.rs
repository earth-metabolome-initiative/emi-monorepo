//! Submodule defining the trait builder implementation for a table and all
//! its extensions.

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{Column, Table, errors::WebCodeGenError};

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
        same_as_network: &crate::ColumnSameAsNetwork,
        conn: &mut PgConnection,
    ) -> Result<(bool, Vec<TokenStream>, Vec<Column>), WebCodeGenError> {
        if self.has_composite_primary_key(conn)? {
            return Ok((false, Vec::new(), Vec::new()));
        }

        // The column may have a same-as relationship
        // with attributes of the current table.
        //
        // Here follow two simple ASCII illustrations for the case where the column is
        // from a direct ancestor and the case where the column is a
        // grand-parent ancestor table:
        //
        // 1. Direct ancestor SAME-AS relationship:
        //
        //   +-------------------+           +-------------------+
        //   |   ParentTable     |           |   ChildTable      |
        //   |-------------------|           |-------------------|
        //   | primary_key       |<-extends->| primary_key       |
        //   | parent_column (*) |<-same-as->| child_column (*)  |
        //   +-------------------+           +-------------------+
        //
        //   (*) SAME-AS relationship between `parent_column` and `child_column`
        //
        // 2. Grand-parent ancestor SAME-AS relationship:
        //
        //   +---------------------+
        //   |  GrandParentTable   |
        //   |---------------------|
        //   | primary_key         |
        //   | gra_parent_col (*)  |
        //   +---------------------+
        //            ^
        //            |
        //        extends
        //            |
        //   +---------------------+
        //   |    ParentTable      |
        //   |---------------------|
        //   | primary_key         |
        //   | parent_column       |
        //   +---------------------+
        //            ^
        //            |
        //        extends
        //            |
        //   +---------------------+
        //   |    ChildTable       |
        //   |---------------------|
        //   | primary_key         |
        //   | child_column (*)    |
        //   +---------------------+
        //
        //   (*) SAME-AS relationship between `gra_parent_col` and `child_column`
        //

        let current_column_ident = current_column.snake_case_ident()?;
        let current_column_camel_case_ident = current_column.camel_case_ident()?;
        let mut assignments = Vec::new();
        let mut requires_mutability = false;
        let mut involved_columns: Vec<Column> = Vec::new();
        let required_ancestor_columns: Vec<&Column> = same_as_network
            .ancestral_same_as_columns(self, Some(current_column), true, conn)?
            .into_iter()
            .map(|(_, dst_column)| dst_column)
            .collect();
        involved_columns.extend(required_ancestor_columns.iter().map(|c| (*c).clone()));

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

            let foreign_key_ident = foreign_key.constraint_ident(conn)?;
            let column_setter = required_ancestor_column.getter_ident()?;

            // If the current column is not nullable and the foreign column is, we
            // need to wrap the current column in `Some(...)` to match the type.
            let wrapped_current_column_ident =
                if !current_column.is_nullable() && required_ancestor_column.is_nullable() {
                    quote! { Some(#current_column_ident) }
                } else {
                    quote! { #current_column_ident }
                };

            // The current column is in a same-as relationship with a column
            // from a direct ancestor table.
            assignments.push(quote! {
                self.#foreign_key_ident = self.#foreign_key_ident.#column_setter(#wrapped_current_column_ident).map_err(|err| {
                    err.into_field_name(|attribute| Self::Attributes::Extension(attribute.into()))
                })?;
            });
        }

        for (local_column, foreign_key) in self.partial_builder_columns(conn)? {
            let foreign_table = foreign_key.foreign_table(conn)?.expect(
                "Partial builder foreign keys must have a foreign table, as they cannot be self-referential",
            );
            let foreign_builder = foreign_table.insertable_builder_ty()?;
            let foreign_table_trait = foreign_table.builder_trait_ty()?;

            let local_column_ident = local_column.snake_case_ident()?;
            let local_column_camel_case_ident = local_column.camel_case_ident()?;

            for (_, destination_column) in same_as_network.same_as_columns(
                &self,
                &foreign_table,
                Some(current_column),
                &local_column,
                conn,
            )? {
                involved_columns.push(destination_column.clone());
                let foreign_column_setter_ident = destination_column.getter_ident()?;

                let wrapper_current_column_ident =
                    if !current_column.is_nullable() && destination_column.is_nullable() {
                        quote! { Some(#current_column_ident) }
                    } else {
                        quote! { #current_column_ident }
                    };

                assignments.push(quote! {
                    self.#local_column_ident = <#foreign_builder as #foreign_table_trait>::#foreign_column_setter_ident(
                        self.#local_column_ident,
                        #wrapper_current_column_ident
                    ).map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::#local_column_camel_case_ident(attribute)
                        })
                    })?;
                });
            }
        }

        if let Some(foreign_key) = current_column.requires_partial_builder(conn)? {
            // Furthermore, the column is also associate with an associated table which
            // requires a partial builder as it depends on the primary key of an ancestor
            // table. This makes it necessary for us to check whether
            // there exist any inferred same-as links that go from the
            // current table to the ancestral associated table, going
            // through the specified column.
            //
            // Here follows an ASCII illustration of the case where the column A2 is
            // from an ancestor table A with primary key A1, and there
            // exists an inferred same-as relationship from a column B2
            // in the current table B, with primary key B1, and a column
            // C2 in the ancestor associated table C with primary key C1
            // which also has another foreign key dependency from a
            // column C3 to a primary column D1 in ancestor table D. The
            // extended tables are shown vertically, with the ancestor
            // associated table on the right side. Column A1 extends D1,
            // and column B1 extends A1. Column C3 is a foreign key to
            // column D1 in ancestor table D.
            //
            //   +-------------------+
            //   |      Table D      |
            //   |-------------------|
            //   | D1 (PK)           |
            //   +-------------------+
            //            ^
            //            |
            //   +-------------------+
            //   |      Table A      |
            //   |-------------------|
            //   | A1 (PK, FK->D1)   |
            //   | A2 (FK -> C1)     |
            //   +-------------------+
            //            ^
            //            |
            //   +-------------------+
            //   |      Table B      |
            //   |-------------------|
            //   | B1 (PK, FK->A1)   |
            //   | B2 (*)            |
            //   +-------------------+
            //
            //   +-------------------+
            //   |      Table C      |
            //   |   (associated)    |
            //   |-------------------|
            //   | C1 (PK)           |
            //   | C2 (*)            |
            //   | C3 (FK->D1)       |
            //   +-------------------+
            //
            //  (*) SAME-AS or inferred SAME-AS relationships between A2, B2, and C2.
            //      C3 is a foreign key to D1.

            let foreign_table = foreign_key.foreign_table(conn)?.unwrap();
            let current_column_table = current_column.table(conn)?;

            let same_as_columns = same_as_network.same_as_columns(
                &self,
                &foreign_table,
                None,
                &current_column,
                conn,
            )?;

            for (local_column, foreign_column) in same_as_columns {
                // If the local column is a primary key that is automatically generated,
                // we skip it as it is not settable.
                if local_column.is_part_of_primary_key(conn)?
                    || foreign_column.is_part_of_primary_key(conn)?
                {
                    continue;
                }

                involved_columns.push(local_column.clone());
                involved_columns.push(foreign_column.clone());
                let local_column_ident = local_column.snake_case_ident()?;
                let local_column_camel_case_ident = local_column.camel_case_ident()?;
                let foreign_builder = foreign_table.insertable_builder_ty()?;
                let foreign_table_trait = foreign_table.builder_trait_ty()?;
                let foreign_column_ident = foreign_column.snake_case_ident()?;
                let foreign_column_setter_ident = foreign_column.getter_ident()?;

                // In some cases, the foreign column may be from a table extended
                // by the `current_column_table`, so we need to fully determine the
                // path to the foreign column.
                let foreign_keys_path = extension_network.extension_foreign_keys_path(
                    &current_column_table,
                    foreign_column,
                    conn,
                );

                let maybe_associated_table_path = if let Some(path) = foreign_keys_path {
                    let mut full_path = TokenStream::new();
                    for key in path {
                        let key_ident = key.constraint_ident(conn)?;
                        full_path.extend(quote! { .#key_ident });
                    }
                    Some(full_path)
                } else {
                    None
                };

                requires_mutability = true;

                assignments.push(quote! {
                    if let (Some(local), Some(foreign)) = (self.#local_column_ident, #current_column_ident #maybe_associated_table_path.#foreign_column_ident) {
                        if local != foreign {
                            return Err(
                                web_common_traits::database::InsertError::BuilderError(
                                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                        Self::Attributes::#local_column_camel_case_ident
                                    )
                                )
                            );
                        }
                    } else if let Some(#foreign_column_ident) = #current_column_ident #maybe_associated_table_path.#foreign_column_ident {
                        self.#local_column_ident = Some(#foreign_column_ident);
                    } else if let Some(local) = self.#local_column_ident {
                        #current_column_ident = <#foreign_builder as #foreign_table_trait>::#foreign_column_setter_ident(
                            #current_column_ident,
                            local
                        ).map_err(|e| {
                            e.into_field_name(|attribute| {
                                Self::Attributes::#current_column_camel_case_ident(attribute)
                            })
                        })?;
                    }
                });
            }
        }
        Ok((requires_mutability, assignments, involved_columns))
    }
}
