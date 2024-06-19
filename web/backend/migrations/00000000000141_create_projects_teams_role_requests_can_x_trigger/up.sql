-- The function `can_update_projects_teams_role_requests` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_update function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the projects_teams_role_requests_users_roles table or
-- the projects_teams_role_requests_teams_users table with an appropriate role id.
CREATE FUNCTION can_update_projects_teams_role_requests(author_user_id INTEGER, this_projects_teams_role_requests_table_id INTEGER, this_projects_teams_role_requests_team_id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    this_table_id INTEGER;
    this_team_id INTEGER;
    this_created_by INTEGER;
BEGIN
-- If the author_user_id is NULL, we return FALSE.
    IF author_user_id IS NULL THEN
        RAISE EXCEPTION 'The author_user_id cannot be NULL.';
    END IF;
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT table_id, team_id, created_by, 1 INTO this_table_id, this_team_id, this_created_by, canary FROM projects_teams_role_requests WHERE projects_teams_role_requests.table_id = this_projects_teams_role_requests_table_id AND projects_teams_role_requests.team_id = this_projects_teams_role_requests_team_id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN FALSE;
    END IF;
-- We check whether the user is the created_by of the row.
    IF author_user_id = this_created_by THEN
        RETURN TRUE;
    END IF;
        IF can_update_projects(author_user_id, this_table_id) THEN
            RETURN TRUE;
        END IF;
        IF can_update_teams(author_user_id, this_team_id) THEN
            RETURN TRUE;
        END IF;
    RETURN FALSE;
END;
$$
LANGUAGE plpgsql PARALLEL SAFE;

-- The function `can_admin_projects_teams_role_requests` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_delete function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the projects_teams_role_requests_users_roles table or
-- the projects_teams_role_requests_teams_users table with an appropriate role id.
CREATE FUNCTION can_admin_projects_teams_role_requests(author_user_id INTEGER, this_projects_teams_role_requests_table_id INTEGER, this_projects_teams_role_requests_team_id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    this_table_id INTEGER;
    this_team_id INTEGER;
    this_created_by INTEGER;
BEGIN
-- If the author_user_id is NULL, we return FALSE.
    IF author_user_id IS NULL THEN
        RAISE EXCEPTION 'The author_user_id cannot be NULL.';
    END IF;
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT table_id, team_id, created_by, 1 INTO this_table_id, this_team_id, this_created_by, canary FROM projects_teams_role_requests WHERE projects_teams_role_requests.table_id = this_projects_teams_role_requests_table_id AND projects_teams_role_requests.team_id = this_projects_teams_role_requests_team_id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN FALSE;
    END IF;
-- We check whether the user is the created_by of the row.
    IF author_user_id = this_created_by THEN
        RETURN TRUE;
    END IF;
        IF can_delete_projects(author_user_id, this_table_id) THEN
            RETURN TRUE;
        END IF;
        IF can_delete_teams(author_user_id, this_team_id) THEN
            RETURN TRUE;
        END IF;
    RETURN FALSE;
END;
$$
LANGUAGE plpgsql PARALLEL SAFE;

-- The function `can_view_projects_teams_role_requests` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_view function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the projects_teams_role_requests_users_roles table or
-- the projects_teams_role_requests_teams_users table with an appropriate role id.
CREATE FUNCTION can_view_projects_teams_role_requests(author_user_id INTEGER, this_projects_teams_role_requests_table_id INTEGER, this_projects_teams_role_requests_team_id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    this_table_id INTEGER;
    this_team_id INTEGER;
    this_created_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT table_id, team_id, created_by, 1 INTO this_table_id, this_team_id, this_created_by, canary FROM projects_teams_role_requests WHERE projects_teams_role_requests.table_id = this_projects_teams_role_requests_table_id AND projects_teams_role_requests.team_id = this_projects_teams_role_requests_team_id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN FALSE;
    END IF;
-- We check whether the user is the created_by of the row.
    IF author_user_id = this_created_by THEN
        RETURN TRUE;
    END IF;
        IF can_view_projects(author_user_id, this_table_id) THEN
            RETURN TRUE;
        END IF;
        IF can_view_teams(author_user_id, this_team_id) THEN
            RETURN TRUE;
        END IF;
    RETURN FALSE;
END;
$$
LANGUAGE plpgsql PARALLEL SAFE;

