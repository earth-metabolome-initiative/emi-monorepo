//! Submodule defining methods to create the SQL functions and related triggers
//! to determine whether a user can insert, delete, or update a row in a table.
//!
//! Rows in tables characterized by `created_by` and `updated_by` columns are
//! considered editables by the user who created them and/or by the user who
//! last updated them. Most commonly, such tables will have associated user
//! roles tables which determine additional permissions for each user, plus
//! analogous team roles tables which determine permissions for each team.
//!
//! When a table has a `parent` table (e.g., `samples` has a parent table
//! `projects`), the permissions for the parent table are inherited by the child
//! table. For example, if a user has `insert` permission on a project, they
//! will also have `insert` permission on all samples associated with that
//! project. A parent table is determined as a foreign key reference to another
//! table.
//!
//! Tables with multiple parent tables are at this time not supported and will
//! raise an error.

use diesel::{connection::SimpleConnection, PgConnection};

use crate::{errors::WebCodeGenError, Column, Table};

#[derive(Debug, Default, Clone)]
/// Builder for the authorization functions.
pub struct AuthorizationFunctionBuilder {
    childless_tables: Vec<Table>,
}

impl AuthorizationFunctionBuilder {
    #[must_use]
    /// Adds a new table to the list of childless tables.
    ///
    /// # Arguments
    ///
    /// * `table` - The table to add.
    ///
    /// # Returns
    ///
    /// * `Self` - The updated builder.
    pub fn add_childless_table(mut self, table: Table) -> Self {
        self.childless_tables.push(table);
        self
    }

    /// Returns whether the table can be a parent table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The connection to the database.
    fn can_be_parent(&self, table: &Table) -> bool {
        self.childless_tables.contains(table)
    }

    /// Returns the parent tab
    ///
    /// # Arguments
    ///
    /// * `conn` - The connection to the database.
    fn parent_tables(
        &self,
        table: &Table,
        conn: &mut PgConnection,
    ) -> Result<Vec<(Column, Table, Column)>, WebCodeGenError> {
        let mut parent_tables = Vec::new();
        for foreign_key in table.foreign_keys(conn)? {
            let Some((parent_table, parent_column)) = foreign_key.foreign_table(conn)? else {
                continue;
            };
            if self.can_be_parent(&parent_table) {
                parent_tables.push((foreign_key, parent_table, parent_column));
            }
        }
        Ok(parent_tables)
    }

    /// Creates in the database for all tables the authorization functions and
    /// triggers.
    ///
    /// # Arguments
    ///
    /// * `conn` - The connection to the database.
    /// * `table_catalog` - The catalog of the tables to create the functions
    ///   for.
    /// * `table_schema` - The schema of the tables to create the functions for.
    ///
    /// # Errors
    ///
    /// * If an error occurs while creating the functions and triggers
    /// * If the roles mechanism is incomplete
    /// * If the roles mechanism is not complete
    /// * If the table does not have all the roles mechanism tables
    /// * If the table does not have a primary key column
    /// * If the table does not have a column named `updated_by`
    /// * If the table does not have a column named `created_by`
    pub fn create_authorization_functions_and_triggers(
        &self,
        conn: &mut PgConnection,
        table_catalog: &str,
        table_schema: Option<&str>,
    ) -> Result<(), WebCodeGenError> {
        for table in Table::load_all_topologically(conn, table_catalog, table_schema)? {
            if self.childless_tables.contains(&table) {
                continue;
            }
            if !table.has_all_roles_mechanism_tables(conn) {
                continue;
            }
            let function = self.canx_function_and_trigger(&table, conn)?;
            if let Err(err) = conn.batch_execute(&function) {
                println!("Failed running SQL: {function}");
                return Err(WebCodeGenError::DieselError(err));
            }
        }
        Ok(())
    }

    /// Returns the authorization function for the table.
    ///
    /// # Arguments
    ///
    /// * `table` - The table for which to create the function.
    /// * `conn` - The connection to the database.
    pub fn canx_function_and_trigger(
        &self,
        table: &Table,
        conn: &mut PgConnection,
    ) -> Result<String, WebCodeGenError> {
        let primary_keys = table.primary_key_columns(conn)?;
        let user_table =
            Table::load(conn, "users", Some(&table.table_schema), &table.table_catalog)?;
        let user_id_column = user_table
            .primary_key_columns(conn)?
            .first()
            .cloned()
            .ok_or(WebCodeGenError::NoPrimaryKeyColumn(Box::new(user_table)))?;
        let arguments = primary_keys
            .iter()
            .map(|column| format!("this_{} {}", column.column_name, column.raw_data_type()))
            .collect::<Vec<String>>()
            .join(",\n\t");
        let parent_tables = self.parent_tables(table, conn)?;

        let mut columns_to_retrieve = parent_tables
            .iter()
            .map(|(foreign_key, _parent_table, _parent_column)| foreign_key.clone())
            .collect::<Vec<Column>>();

        let updator_check = if let Ok(updated_by) = table.column_by_name(conn, "updated_by") {
            let updator_check = "IF session_user_id = this_updated_by THEN
        RETURN TRUE;
    END IF;
            "
            .to_owned();
            if !columns_to_retrieve.contains(&updated_by) {
                columns_to_retrieve.push(updated_by);
            }
            updator_check
        } else {
            String::new()
        };
        let creator_check = if let Ok(created_by) = table.column_by_name(conn, "created_by") {
            let creator_check = "IF session_user_id = this_created_by THEN
        RETURN TRUE;
    END IF;
            "
            .to_owned();
            if !columns_to_retrieve.contains(&created_by) {
                columns_to_retrieve.push(created_by);
            }
            creator_check
        } else {
            String::new()
        };

        let roles_table_check = if table.requires_roles_table(conn)? {
            // We check that all of the tables necessary for the roles tables are present.
            if !table.has_all_roles_mechanism_tables(conn) {
                return Err(WebCodeGenError::RolesMechanismIncomplete(Box::new(table.clone())));
            }

            let users_where_statement = primary_keys
                .iter()
                .map(|column| {
                    format!(
                        "{}_users_roles.{} = this_{}",
                        table.table_name, column.column_name, column.column_name
                    )
                })
                .collect::<Vec<String>>()
                .join(" AND ");

            let teams_where_statement = primary_keys
                .iter()
                .map(|column| {
                    format!(
                        "{}_teams_roles.{} = this_{}",
                        table.table_name, column.column_name, column.column_name
                    )
                })
                .collect::<Vec<String>>()
                .join(" AND ");

            format!(
                "IF EXISTS (
        SELECT 1 FROM {table_name}_users_roles
        WHERE
            {table_name}_users_roles.user_id = session_user_id AND
            {table_name}_users_roles.role_id >= level_required AND
            {users_where_statement}
    ) THEN
        RETURN TRUE;
    END IF;
    IF EXISTS (
        SELECT 1 FROM {table_name}_teams_roles
        JOIN team_members ON team_members.team_id = {table_name}_teams_roles.team_id
        WHERE
            team_members.user_id = session_user_id AND
            {table_name}_teams_roles.role_id >= level_required AND
            {teams_where_statement}
    ) THEN
        RETURN TRUE;
    END IF;
            ",
                table_name = table.table_name
            )
        } else {
            String::new()
        };

        let parent_arguments = format!(
            "{};",
            columns_to_retrieve
                .iter()
                .map(|column| {
                    format!(
                        "this_{column_name} {data_type}",
                        column_name = column.column_name,
                        data_type = column.raw_data_type()
                    )
                })
                .collect::<Vec<String>>()
                .join(";\n")
        );

        let parent_arguments_raw_names = columns_to_retrieve
            .iter()
            .map(|column| column.column_name.clone())
            .collect::<Vec<String>>()
            .join(", ");

        let this_parent_arguments_raw_names = columns_to_retrieve
            .iter()
            .map(|column| format!("this_{}", column.column_name))
            .collect::<Vec<String>>()
            .join(", ");

        let where_statement = primary_keys
            .iter()
            .map(|column| {
                format!("{}.{} = this_{}", table.table_name, column.column_name, column.column_name)
            })
            .collect::<Vec<String>>()
            .join(" AND ");

        let parent_tables_check = parent_tables
            .iter()
            .map(|(foreign_key, parent_table, _parent_column)| {
                // If the user has the required level on the parent table, we return TRUE.
                format!(
                    "IF canx_{parent_table_name}(session_user_id, this_{parent_column_name}, level_required) THEN
        RETURN TRUE;
    END IF;
                ",
                    parent_table_name = parent_table.table_name,
                    parent_column_name = foreign_key.column_name
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let new_row_primary_key_arguments = primary_keys
            .iter()
            .map(|column| format!("NEW.{}", column.column_name))
            .collect::<Vec<String>>()
            .join(", ");

        let insert_check = if parent_tables.is_empty() {
            String::new()
        } else {
            let parent_check = parent_tables.iter().map(|(foreign_key, parent_table, _parent_column)| {
            format!(
                "IF NOT canx_{parent_table_name}(NEW.created_by, NEW.{parent_column_name}, 2) THEN
        RAISE EXCEPTION 'unauthorized_update' USING DETAIL = 'Unauthorized to insert this row of the `{table_name}` table.';
    END IF;
                ",
                table_name = table.table_name,
                parent_table_name = parent_table.table_name,
                parent_column_name = foreign_key.column_name
            )
        }).collect::<Vec<String>>().join("\n");

            format!(
                "
            IF TG_OP = 'INSERT' THEN
        {parent_check}
    END IF;
            "
            )
        };

        Ok(format!(
            "
CREATE OR REPLACE FUNCTION canx_{table_name}(
    session_user_id {session_user_id_type},
    {arguments},
    level_required SMALLINT
)
RETURNS BOOLEAN
STRICT 
AS $$
DECLARE
    canary INTEGER;
    {parent_arguments}
BEGIN
    -- We retrieve the informations on the row.
    SELECT {parent_arguments_raw_names}, 1 INTO {this_parent_arguments_raw_names}, canary FROM {table_name} WHERE {where_statement};
    -- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN FALSE;
    END IF;
    {creator_check}
    {updator_check}
    {roles_table_check}
    {parent_tables_check}
    -- If none of the previous checks returned TRUE, we return FALSE.
    RETURN FALSE;
END;
$$
LANGUAGE plpgsql PARALLEL SAFE;
CREATE OR REPLACE FUNCTION {trigger_function_name}()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'UPDATE' THEN
        IF NOT canx_{table_name}(
            NEW.updated_by,
            {new_row_primary_key_arguments},
            2
        ) THEN
            RAISE EXCEPTION 'unauthorized_update' USING DETAIL = 'Unauthorized to update this row of the `{table_name}` table.';
        END IF;
    END IF;
    {insert_check}
    RETURN NEW;
END;
$$
LANGUAGE plpgsql PARALLEL SAFE;
{update_trigger}
",
            session_user_id_type = user_id_column.raw_data_type(),
            table_name = table.table_name,
            update_trigger = self.update_trigger(table),
            trigger_function_name = self.trigger_function_name(table),
        ))
    }

    fn trigger_function_name(&self, table: &Table) -> String {
        format!("can_insert_or_update_{table_name}_trigger", table_name = table.table_name)
    }

    /// Returns the trigger to call when the provided table receives an
    /// operation.
    fn update_trigger(&self, table: &Table) -> String {
        format!(
            "CREATE OR REPLACE TRIGGER can_insert_or_update_{table_name}
BEFORE INSERT OR UPDATE ON {table_name}
FOR EACH ROW
EXECUTE FUNCTION {trigger_function_name}();
",
            table_name = table.table_name,
            trigger_function_name = self.trigger_function_name(table)
        )
    }
}
