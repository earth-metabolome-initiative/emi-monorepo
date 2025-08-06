//! Submodule handling the same-as and inferred sane-as relationships for
//! insertable setter methods.

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{Codegen, Column, Table, errors::WebCodeGenError};

impl Codegen<'_> {
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
    pub(super) fn generate_same_as_assigments(
        &self,
        table: &Table,
        current_column: &Column,
        conn: &mut PgConnection,
    ) -> Result<(Vec<TokenStream>, Vec<Column>), WebCodeGenError> {
        // The column may have a same-as relationship
        // with attributes of the current table.
        //
        // Here follow two simple ASCII illustrations for the case where the column is
        // from a direct ancestor and the case where the column is a
        // grand-parent ancestor table:
        //
        // 1. SAME-AS relationship within the current table (which is unusual but not
        //    impossible):
        //
        //   +-------------------+
        //   |   CurrentTable    |
        //   |-------------------|
        //   | primary_key       |
        //   | column_name (*)   |
        //   | same_as_column (*)|
        //   +-------------------+
        //
        //  (*) SAME-AS relationship between `column_name` and `same_as_column`
        //
        // 2. Direct ancestor SAME-AS relationship:
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
        // 3. Grand-parent ancestor SAME-AS relationship:
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
        let mut assignments = Vec::new();

        // We iterate over the columns of the current table and we identify whether
        // any of them has a same-as relationship with the current column.
        for local_column in table.columns(conn)? {
            let same_as_columns = local_column.same_as_columns(conn)?;
            if same_as_columns.contains(current_column) {
                let local_column_ident = local_column.snake_case_ident()?;
                let local_column_setter_ident = local_column.getter_ident()?;
                let current_column_enum_path =
                    self.column_enum_path(table, current_column, None, conn)?;
                assignments.push(quote! {
                    /// If the local column is already set, we check whether it matches the
                    /// current column.
                    if let Some(local) = self.#local_column_ident {
                        /// If the two values are not the same, we return an error.
                        if local != #current_column_ident {
                            return Err(
                                web_common_traits::database::InsertError::BuilderError(
                                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                        #current_column_enum_path
                                    )
                                )
                            );
                        }
                    } else {
                        /// Otherwise, if the local column is not set, we assign the
                        /// current column value to the local column.
                        self.#local_column_setter_ident = Some(#current_column_ident)?;
                    }
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

            // We determine the foreign keys from the current table to the current column,
            // if any.
            let current_column_foreign_keys = self
                .table_extension_network()
                .unwrap()
                .extension_foreign_keys_path(table, current_column, conn);
            let self_extension_path_ident = if let Some(path) = current_column_foreign_keys {
                let mut full_path = TokenStream::new();
                for key in path {
                    let key_ident = key.constraint_ident(conn)?;
                    full_path.extend(quote! { .#key_ident });
                }
                quote! {
                    self.#full_path
                }
            } else {
                quote! {
                    self.#current_column_ident
                }
            };

            for (local_column, foreign_column) in self
                .column_same_as_network()
                .unwrap()
                .inferred_same_as_columns(&table, &foreign_table, &current_column)
            {
                let local_column_ident = local_column.snake_case_ident()?;
                let foreign_column_ident = foreign_column.snake_case_ident()?;
                let foreign_column_setter_ident = foreign_column.getter_ident()?;
                let current_column_enum_path =
                    self.column_enum_path(table, foreign_column, Some(&current_column), conn)?;

                // In some cases, the foreign column may be from a table extended
                // by the `current_column_table`, so we need to fully determine the
                // path to the foreign column.
                let foreign_keys_path = self
                    .table_extension_network()
                    .unwrap()
                    .extension_foreign_keys_path(&current_column_table, foreign_column, conn);

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

                assignments.push(quote! {
                    if let (Some(local), Some(foreign)) = (self.#local_column_ident, #current_column_ident #maybe_associated_table_path.#foreign_column_ident) {
                        if local != foreign {
                            return Err(
                                web_common_traits::database::InsertError::BuilderError(
                                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                        #current_column_enum_path
                                    )
                                )
                            );
                        }
                    } else if let Some(foreign) = #current_column_ident #maybe_associated_table_path.#foreign_column_ident {
                        self.#local_column_ident = Some(foreign)?;
                    } else if let Some(local) = self.#local_column_ident {
                        #self_extension_path_ident #maybe_associated_table_path.#foreign_column_ident = #self_extension_path_ident #maybe_associated_table_path.#foreign_column_ident.#foreign_column_setter_ident(local)?;
                    }
                });
            }
        }
        Ok((assignments, Vec::new()))
    }
}
