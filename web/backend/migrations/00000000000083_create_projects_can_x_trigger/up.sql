-- The function `can_update_projects` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_update function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the projects_users_roles table or
-- the projects_teams_users table with an appropriate role id.
CREATE FUNCTION can_update_projects(author_user_id INTEGER, this_projects_id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    this_parent_project_id INTEGER;
    this_created_by INTEGER;
    this_updated_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT parent_project_id, created_by, updated_by, 1 INTO this_parent_project_id, this_created_by, this_updated_by, canary FROM projects WHERE projects.id = this_projects_id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN TRUE;
    END IF;
-- If the author_user_id is NULL, we return FALSE.
    IF author_user_id IS NULL THEN
        RETURN FALSE;
    END IF;
-- We check whether the user is the created_by of the row.
    IF author_user_id = this_created_by THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is the updated_by of the row.
    IF author_user_id = this_updated_by THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is in the projects_users_roles table with an appropriate role id.
    IF EXISTS (SELECT 1 FROM projects_users_roles WHERE projects_users_roles.user_id = author_user_id AND projects_users_roles.role_id <= 2 AND projects_users_roles.table_id == this_projects_id) THEN
        RETURN TRUE;
    END IF;
-- If the parent column is not NULL, we call the can_update function of the parent column to determine whether the user can edit the row.
    IF this_parent_project_id IS NOT NULL THEN
        IF NOT can_update_projects(author_user_id, this_parent_project_id) THEN
            RETURN FALSE;
        END IF;
    END IF;
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;

-- The function `can_update_projects_trigger` is a trigger function that checks whether the user can update the row.
CREATE FUNCTION can_update_projects_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'UPDATE' THEN
        IF NOT can_update_projects(NEW.updated_by, NEW.id) THEN
            RAISE EXCEPTION 'The user does not have the permission to update this row.';
        END IF;
    END IF;
-- We check whether the user can update the row.
    IF TG_OP = 'INSERT' AND NEW.parent_project_id IS NOT NULL THEN
        IF NOT can_update_projects(NEW.created_by, NEW.parent_project_id) THEN
            RAISE EXCEPTION 'The user does not have the permission to update this row.';
        END IF;
    END IF;
    RETURN NEW;
END;
$$
LANGUAGE plpgsql;

-- We create a trigger that calls the `can_update_projects` function before each INSERT or UPDATE.
CREATE TRIGGER can_update_projects
BEFORE INSERT OR UPDATE ON projects
FOR EACH ROW
EXECUTE FUNCTION can_update_projects_trigger();
-- The function `can_admin_projects` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_delete function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the projects_users_roles table or
-- the projects_teams_users table with an appropriate role id.
CREATE FUNCTION can_admin_projects(author_user_id INTEGER, this_projects_id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    this_parent_project_id INTEGER;
    this_created_by INTEGER;
    this_updated_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT parent_project_id, created_by, updated_by, 1 INTO this_parent_project_id, this_created_by, this_updated_by, canary FROM projects WHERE projects.id = this_projects_id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN TRUE;
    END IF;
-- If the author_user_id is NULL, we return FALSE.
    IF author_user_id IS NULL THEN
        RETURN FALSE;
    END IF;
-- We check whether the user is the created_by of the row.
    IF author_user_id = this_created_by THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is the updated_by of the row.
    IF author_user_id = this_updated_by THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is in the projects_users_roles table with an appropriate role id.
    IF EXISTS (SELECT 1 FROM projects_users_roles WHERE projects_users_roles.user_id = author_user_id AND projects_users_roles.role_id <= 1 AND projects_users_roles.table_id == this_projects_id) THEN
        RETURN TRUE;
    END IF;
-- If the parent column is not NULL, we call the can_delete function of the parent column to determine whether the user can edit the row.
    IF this_parent_project_id IS NOT NULL THEN
        IF NOT can_delete_projects(author_user_id, this_parent_project_id) THEN
            RETURN FALSE;
        END IF;
    END IF;
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;

-- The function `can_view_projects` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_view function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the projects_users_roles table or
-- the projects_teams_users table with an appropriate role id.
CREATE FUNCTION can_view_projects(author_user_id INTEGER, this_projects_id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    this_parent_project_id INTEGER;
    this_created_by INTEGER;
    this_updated_by INTEGER;
    this_public BOOLEAN;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT parent_project_id, created_by, updated_by, public, 1 INTO this_parent_project_id, this_created_by, this_updated_by, this_public, canary FROM projects WHERE projects.id = this_projects_id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN TRUE;
    END IF;
-- If the row is public, we return TRUE.
    IF this_public THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is the created_by of the row.
    IF author_user_id = this_created_by THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is the updated_by of the row.
    IF author_user_id = this_updated_by THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is in the projects_users_roles table with an appropriate role id.
    IF EXISTS (SELECT 1 FROM projects_users_roles WHERE projects_users_roles.user_id = author_user_id AND projects_users_roles.role_id <= 3 AND projects_users_roles.table_id == this_projects_id) THEN
        RETURN TRUE;
    END IF;
-- If the parent column is not NULL, we call the can_view function of the parent column to determine whether the user can edit the row.
    IF this_parent_project_id IS NOT NULL THEN
        IF NOT can_view_projects(author_user_id, this_parent_project_id) THEN
            RETURN FALSE;
        END IF;
    END IF;
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;

