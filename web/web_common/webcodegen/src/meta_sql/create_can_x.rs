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
use crate::Table;

impl Table {
    pub fn create_can_view(&self, conn: &mut PgConnection) -> Result<String, WebCodeGenError> {
        let table_name = self.name.to_lowercase();

        let query = format!(
            "CREATE FUNCTION can_view_{table_name}(author_user_id INTEGER, this_{table_name}_id INTEGER)"
        );
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
            -- We check whether the user can update the row.
                IF TG_OP = 'INSERT' THEN
                    IF NOT can_update_projects(NEW.created_by, NEW.project_id) THEN
                        RAISE EXCEPTION 'unauthorized_update' USING DETAIL = 'Unauthorized to update this row of the `{table_name}` table.';
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
