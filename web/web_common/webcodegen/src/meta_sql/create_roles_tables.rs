//! Submodule providing methods to generate Roles tables for all editable tables.
//!
//! Editable tables are the ones characterized by `created_by` and `updated_by` columns.

use diesel::PgConnection;

use crate::errors::WebCodeGenError;
use crate::Table;

impl Table {
    /// Returns whether all tables required for the roles mechanism are present.
    pub fn has_all_roles_mechanism_tables(&self, conn: &mut PgConnection) -> bool {
        self.has_user_roles_table(conn)
            && self.has_team_roles_table(conn)
            && Table::load(conn, "roles", Some(&self.table_schema), &self.table_catalog).is_ok()
            && Table::load(conn, "users", Some(&self.table_schema), &self.table_catalog).is_ok()
            && Table::load(conn, "teams", Some(&self.table_schema), &self.table_catalog).is_ok()
            && Table::load(conn, "teams_users", Some(&self.table_schema), &self.table_catalog).is_ok()
    }

    /// Returns whether the table is expected to have a roles table.
    pub fn requires_roles_table(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        Ok(self.columns(conn)?.iter().any(|c| c.is_created_by(conn)))
    }

    /// Returns whether the table currently has a users_role table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    pub fn has_user_roles_table(&self, conn: &mut PgConnection) -> bool {
        let expected_user_roles_table_name = format!("{}_users_roles", self.table_name);
        Table::load(
            conn,
            &expected_user_roles_table_name,
            Some(&self.table_schema),
            &self.table_catalog,
        )
        .is_ok()
    }

    /// Returns whether the table currently has a teams_roles table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    pub fn has_team_roles_table(&self, conn: &mut PgConnection) -> bool {
        let expected_team_roles_table_name = format!("{}_teams_roles", self.table_name);
        Table::load(
            conn,
            &expected_team_roles_table_name,
            Some(&self.table_schema),
            &self.table_catalog,
        )
        .is_ok()
    }

    fn create_roles_table(
        &self,
        reference_table: &Table,
        conn: &mut PgConnection,
        roles_table_type: &str,
    ) -> Result<String, WebCodeGenError> {
        if reference_table.primary_key_columns(conn)?.len() != 1 {
            return Err(WebCodeGenError::IllegalTable(
                "Roles table can only be created for tables with a single primary key column"
                    .to_string(),
            ));
        }

        let mut create_table = format!(
            "CREATE TABLE IF NOT EXISTS {}_{}_{}(\n",
            self.table_name, reference_table.table_name, roles_table_type
        );

        let Ok(roles_table) =
            Table::load(conn, "roles", Some(&self.table_schema), &self.table_catalog)
        else {
            return Err(WebCodeGenError::MissingTable("roles".to_string()));
        };

        let Ok(users) = Table::load(conn, "users", Some(&self.table_schema), &self.table_catalog)
        else {
            return Err(WebCodeGenError::MissingTable("users".to_string()));
        };
        let mut primary_key_names = Vec::new();

        for primary_key_column in self.primary_key_columns(conn)? {
            create_table.push_str(&format!(
                "{} {} NOT NULL,\n",
                primary_key_column.column_name,
                primary_key_column.data_type_str(conn)?
            ));
            primary_key_names.push(primary_key_column.column_name.clone());
        }

        create_table.push_str(&format!(
            "{}_id {} NOT NULL,\n",
            reference_table.singular_table_name(),
            reference_table.primary_key_columns(conn)?[0].data_type_str(conn)?
        ));

        create_table.push_str(&format!(
            "role_id {} NOT NULL,\n",
            roles_table.primary_key_columns(conn)?[0].data_type_str(conn)?
        ));

        create_table.push_str(&format!(
            "created_by {} NOT NULL,\n",
            users.primary_key_columns(conn)?[0].data_type_str(conn)?
        ));

        create_table.push_str("created_at TIMESTAMP NOT NULL DEFAULT NOW(),\n");

        // We build the primary key using the primary key columns of the reference table
        create_table.push_str(&format!(
            "PRIMARY KEY ({}, {}_id),\n",
            primary_key_names.join(", "),
            reference_table.singular_table_name(),
        ));

        create_table.push_str(&format!(
            "FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE CASCADE,\n",
            primary_key_names.join(", "),
            self.table_name,
            primary_key_names.join(", "),
        ));

        create_table.push_str(&format!(
            "FOREIGN KEY ({}_id) REFERENCES {}({}) ON DELETE CASCADE,\n",
            reference_table.singular_table_name(),
            reference_table.table_name,
            reference_table.primary_key_columns(conn)?[0].column_name
        ));

        create_table.push_str("FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,\n");

        create_table.push_str("FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE\n");

        create_table.push_str(");\n");

        Ok(create_table)
    }

    /// Generates the SQL code to create the roles tables for all editable tables.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// If an error occurs while creating the roles tables
    pub fn create_roles_tables(&self, conn: &mut PgConnection) -> Result<String, WebCodeGenError> {
        if !self.requires_roles_table(conn)? {
            return Err(WebCodeGenError::IllegalRolesTable(format!(
                "Table {} does not require a roles table",
                self.table_name
            )));
        }

        let role_table_types = ["roles", "role_requests", "role_invitations"];
        let mut reference_tables = Vec::new();
        if let Ok(reference_table) =
            Table::load(conn, "users", Some(&self.table_schema), &self.table_catalog)
        {
            reference_tables.push(reference_table);
        } else {
            return Err(WebCodeGenError::MissingTable("users".to_string()));
        }

        if let Ok(reference_table) =
            Table::load(conn, "teams", Some(&self.table_schema), &self.table_catalog)
        {
            reference_tables.push(reference_table);
        } else {
            return Err(WebCodeGenError::MissingTable("teams".to_string()));
        }

        let mut create_tables = String::new();
        for reference_table in reference_tables {
            for role_table_type in &role_table_types {
                create_tables.push_str(&self.create_roles_table(
                    &reference_table,
                    conn,
                    role_table_type,
                )?);
                create_tables.push_str("\n\n");
            }
        }

        Ok(create_tables)
    }
}
