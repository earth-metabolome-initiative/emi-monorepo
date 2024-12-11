//! Submodule to generate the CanX PostgreSQL functions to validate whether a user is allowed to perform an operation.

// -- The function `can_view_nameplates` takes a user ID (INTEGER) and the primary keys
// -- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
// -- may depend on the parent column, this function retrieves the value of the parent column from the row
// -- and calls the parent column's can_view function if the parent column is not NULL. Otherwise, the function
// -- checks if the row was created by the user or if the user is found in either the nameplates_users_roles table or
// -- the nameplates_teams_users table with an appropriate role id.
// CREATE FUNCTION can_view_nameplates(author_user_id INTEGER, this_nameplates_id INTEGER)
// RETURNS BOOLEAN AS $$
// DECLARE
//     canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
//     this_project_id INTEGER;
//     this_created_by INTEGER;
//     this_updated_by INTEGER;
// BEGIN
// -- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
//     SELECT project_id, created_by, updated_by, 1 INTO this_project_id, this_created_by, this_updated_by, canary FROM nameplates WHERE nameplates.id = this_nameplates_id;
// -- If the row does not exist, we return FALSE.
//     IF canary IS NULL THEN
//         RETURN FALSE;
//     END IF;
// -- We check whether the user is the created_by of the row.
//     IF author_user_id = this_created_by THEN
//         RETURN TRUE;
//     END IF;
// -- We check whether the user is the updated_by of the row.
//     IF author_user_id = this_updated_by THEN
//         RETURN TRUE;
//     END IF;
//         IF NOT can_view_projects(author_user_id, this_project_id) THEN
//             RETURN FALSE;
//         END IF;
//     RETURN TRUE;
// END;
// $$
// LANGUAGE plpgsql PARALLEL SAFE;

// -- The function `can_admin_nameplates` takes a user ID (INTEGER) and the primary keys
// -- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
// -- may depend on the parent column, this function retrieves the value of the parent column from the row
// -- and calls the parent column's can_admin function if the parent column is not NULL. Otherwise, the function
// -- checks if the row was created by the user or if the user is found in either the nameplates_users_roles table or
// -- the nameplates_teams_users table with an appropriate role id.
// CREATE FUNCTION can_admin_nameplates(author_user_id INTEGER, this_nameplates_id INTEGER)
// RETURNS BOOLEAN AS $$
// DECLARE
//     canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
//     this_project_id INTEGER;
//     this_created_by INTEGER;
//     this_updated_by INTEGER;
// BEGIN
// -- If the author_user_id is NULL, we return FALSE.
//     IF author_user_id IS NULL THEN
//         RAISE EXCEPTION 'The author_user_id cannot be NULL.';
//     END IF;
// -- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
//     SELECT project_id, created_by, updated_by, 1 INTO this_project_id, this_created_by, this_updated_by, canary FROM nameplates WHERE nameplates.id = this_nameplates_id;
// -- If the row does not exist, we return FALSE.
//     IF canary IS NULL THEN
//         RETURN FALSE;
//     END IF;
// -- We check whether the user is the created_by of the row.
//     IF author_user_id = this_created_by THEN
//         RETURN TRUE;
//     END IF;
// -- We check whether the user is the updated_by of the row.
//     IF author_user_id = this_updated_by THEN
//         RETURN TRUE;
//     END IF;
//         IF NOT can_admin_projects(author_user_id, this_project_id) THEN
//             RETURN FALSE;
//         END IF;
//     RETURN FALSE;
// END;
// $$
// LANGUAGE plpgsql PARALLEL SAFE;

// -- The function `can_update_nameplates` takes a user ID (INTEGER) and the primary keys
// -- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
// -- may depend on the parent column, this function retrieves the value of the parent column from the row
// -- and calls the parent column's can_update function if the parent column is not NULL. Otherwise, the function
// -- checks if the row was created by the user or if the user is found in either the nameplates_users_roles table or
// -- the nameplates_teams_users table with an appropriate role id.
// CREATE FUNCTION can_update_nameplates(author_user_id INTEGER, this_nameplates_id INTEGER)
// RETURNS BOOLEAN AS $$
// DECLARE
//     canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
//     this_project_id INTEGER;
//     this_created_by INTEGER;
//     this_updated_by INTEGER;
// BEGIN
// -- If the author_user_id is NULL, we return FALSE.
//     IF author_user_id IS NULL THEN
//         RAISE EXCEPTION 'The author_user_id cannot be NULL.';
//     END IF;
// -- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
//     SELECT project_id, created_by, updated_by, 1 INTO this_project_id, this_created_by, this_updated_by, canary FROM nameplates WHERE nameplates.id = this_nameplates_id;
// -- If the row does not exist, we return FALSE.
//     IF canary IS NULL THEN
//         RETURN FALSE;
//     END IF;
// -- We check whether the user is the created_by of the row.
//     IF author_user_id = this_created_by THEN
//         RETURN TRUE;
//     END IF;
// -- We check whether the user is the updated_by of the row.
//     IF author_user_id = this_updated_by THEN
//         RETURN TRUE;
//     END IF;
//         IF NOT can_update_projects(author_user_id, this_project_id) THEN
//             RETURN FALSE;
//         END IF;
//     RETURN FALSE;
// END;
// $$
// LANGUAGE plpgsql PARALLEL SAFE;

use diesel::PgConnection;

use crate::errors::WebCodeGenError;
use crate::{Column, ConstraintError, IsForeignKeyConstraint, Table};

pub struct CanXBuilder<'a> {
    user_table: Table,
    parent_tables: Vec<Table>,
    author_columns: Vec<Column>,
    table_catalog: &'a str,
    conn: &'a mut PgConnection,
}

impl<'a> CanXBuilder<'a> {
    pub fn new<'b>(
        user_table_name: &str,
        table_schema: Option<&str>,
        table_catalog: &'b str,
        conn: &'b mut PgConnection,
    ) -> Result<CanXBuilder<'b>, WebCodeGenError> {
        let user_table =
            if let Some(table) = Table::load(conn, user_table_name, table_schema, table_catalog)? {
                table
            } else {
                return Err(WebCodeGenError::MissingTable(user_table_name.to_string()));
            };

        // Check whether the user table has a primary key composed of a single column.
        if user_table.has_composite_primary_key(conn)? {
            return Err(WebCodeGenError::UnexpectedCompositePrimaryKey(user_table));
        }

        Ok(CanXBuilder {
            user_table,
            parent_tables: Vec::new(),
            table_catalog,
            author_columns: Vec::new(),
            conn,
        })
    }

    /// Adds a parent table to the list of parent tables.
    pub fn add_parent_table(mut self, parent_table: Table) -> Self {
        self.parent_tables.push(parent_table);
        self
    }

    /// Add an author column to the list of author columns.
    ///
    /// # Arguments
    ///
    /// * `author_column` - The column that will be used to determine whether the user can perform an operation on a row.
    ///
    /// # Errors
    ///
    /// * If the column is not a foreign key column that references the user table.
    pub fn add_author_column(mut self, author_column: Column) -> Result<Self, WebCodeGenError> {
        if !author_column
            .foreign_table(&mut self.conn)?
            .map_or(true, |(table, _)| &table != &self.user_table)
        {
            return Err(ConstraintError::NotForeignKeyColumn {
                table: self.user_table.clone(),
                column: author_column.clone(),
            }
            .into());
        }
        self.author_columns.push(author_column);
        Ok(self)
    }

    /// Returns the data type of the user table's primary key.
    fn user_primary_key_type(&mut self) -> Result<String, WebCodeGenError> {
        Ok(self
            .user_table
            .primary_key_columns(&mut self.conn)?
            .first()
            .unwrap()
            .data_type_str()?
            .to_owned())
    }

    pub fn create_can_view(&mut self, table: &Table) -> Result<String, WebCodeGenError> {
        let table_name = &table.table_name;
        let user_primary_key_type = self.user_primary_key_type()?;

        let table_primaty_keys = table
            .primary_key_columns(&mut self.conn)?
            .iter()
            .map(|col| {
                Ok(format!(
                    "{column_name} {column_type}",
                    column_name = col.column_name,
                    column_type = col.data_type_str()?
                ))
            })
            .collect::<Result<Vec<_>, WebCodeGenError>>()?
            .join(",\n\t ");

        // For each author column present in the table,
        // we check whether the 'viewer_id' is the author of the row
        // associated to the primary key(s) provided. If so, we return TRUE
        // as the author is allowed to view the row.
        let author_columns_in_table = table
            .columns(&mut self.conn)?
            .into_iter()
            .filter(|col| {
                self.author_columns.iter().any(|author_col| {
                    author_col.column_name == col.column_name
                        && col
                            .foreign_table(&mut self.conn)
                            .map_or(false, |maybe_table| {
                                maybe_table.map_or(false, |(table, _)| &table == &self.user_table)
                            })
                })
            })
            .collect::<Vec<_>>();

        let query = format!(
            "CREATE FUNCTION can_view_{table_name}(
                viewer_id {user_primary_key_type},
                {table_primaty_keys}
            )
            RETURNS BOOLEAN AS $$
            END;
            $$
            LANGUAGE plpgsql PARALLEL SAFE;
            "
        );

        Ok(query)
    }

    /// Returns the SQL to create the trigger that checks whether the user can update the row.
    pub fn create_can_update_trigger(&self) -> String {
        format!(
            "CREATE FUNCTION can_update_{table_name}_trigger()
            RETURNS TRIGGER AS $$
            BEGIN
                IF TG_OP = 'UPDATE' THEN
                    IF NOT can_update_{table_name}(NEW.updated_by, NEW.id) THEN
                        RAISE EXCEPTION 'unauthorized_update' USING DETAIL = 'Unauthorized to update this row of the `{table_name}` table.';
                    END IF;
                END IF;
                IF TG_OP = 'INSERT' THEN
                    IF NOT can_update_projects(NEW.created_by, NEW.project_id) THEN
                        RAISE EXCEPTION 'unauthorized_insert' USING DETAIL = 'Unauthorized to insert this row of the `{table_name}` table.';
                    END IF;
                END IF;
                RETURN NEW;
            END;
            $$
            LANGUAGE plpgsql PARALLEL SAFE;
            CREATE TRIGGER can_update_{table_name}
            BEFORE INSERT OR UPDATE ON {table_name}
            FOR EACH ROW
            EXECUTE FUNCTION can_update_{table_name}_trigger();",
            table_name = self.table_name
        )
    }
}
