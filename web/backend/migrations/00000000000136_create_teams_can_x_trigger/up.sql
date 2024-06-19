-- The function `can_update_teams` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_update function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the teams_users_roles table or
-- the teams_teams_users table with an appropriate role id.
CREATE FUNCTION can_update_teams(author_user_id INTEGER, this_teams_id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    this_parent_team_id INTEGER;
    this_created_by INTEGER;
    this_updated_by INTEGER;
BEGIN
-- If the author_user_id is NULL, we return FALSE.
    IF author_user_id IS NULL THEN
        RAISE EXCEPTION 'The author_user_id cannot be NULL.';
    END IF;
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT parent_team_id, created_by, updated_by, 1 INTO this_parent_team_id, this_created_by, this_updated_by, canary FROM teams WHERE teams.id = this_teams_id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
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
-- We check whether the user is in the teams_users_roles table with an appropriate role id.
    IF EXISTS (SELECT 1 FROM teams_users_roles WHERE teams_users_roles.user_id = author_user_id AND teams_users_roles.role_id <= 2 AND teams_users_roles.table_id = this_teams_id) THEN
        RETURN TRUE;
    END IF;
    RETURN FALSE;
END;
$$
LANGUAGE plpgsql PARALLEL SAFE;

-- The function `can_update_teams_trigger` is a trigger function that checks whether the user can update the row.
CREATE FUNCTION can_update_teams_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'UPDATE' THEN
        IF NOT can_update_teams(NEW.updated_by, NEW.id) THEN
            RAISE EXCEPTION 'The user does not have the permission to update this row.';
        END IF;
    END IF;
-- We check whether the user can update the row.
    IF TG_OP = 'INSERT' AND NEW.parent_team_id IS NOT NULL THEN
        IF NOT can_update_teams(NEW.created_by, NEW.parent_team_id) THEN
            RAISE EXCEPTION 'The user does not have the permission to update this row.';
        END IF;
    END IF;
    RETURN NEW;
END;
$$
LANGUAGE plpgsql PARALLEL SAFE;

-- We create a trigger that calls the `can_update_teams` function before each INSERT or UPDATE.
CREATE TRIGGER can_update_teams
BEFORE INSERT OR UPDATE ON teams
FOR EACH ROW
EXECUTE FUNCTION can_update_teams_trigger();
-- The function `can_admin_teams` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_delete function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the teams_users_roles table or
-- the teams_teams_users table with an appropriate role id.
CREATE FUNCTION can_admin_teams(author_user_id INTEGER, this_teams_id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    this_parent_team_id INTEGER;
    this_created_by INTEGER;
    this_updated_by INTEGER;
BEGIN
-- If the author_user_id is NULL, we return FALSE.
    IF author_user_id IS NULL THEN
        RAISE EXCEPTION 'The author_user_id cannot be NULL.';
    END IF;
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT parent_team_id, created_by, updated_by, 1 INTO this_parent_team_id, this_created_by, this_updated_by, canary FROM teams WHERE teams.id = this_teams_id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
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
-- We check whether the user is in the teams_users_roles table with an appropriate role id.
    IF EXISTS (SELECT 1 FROM teams_users_roles WHERE teams_users_roles.user_id = author_user_id AND teams_users_roles.role_id <= 1 AND teams_users_roles.table_id = this_teams_id) THEN
        RETURN TRUE;
    END IF;
    RETURN FALSE;
END;
$$
LANGUAGE plpgsql PARALLEL SAFE;

