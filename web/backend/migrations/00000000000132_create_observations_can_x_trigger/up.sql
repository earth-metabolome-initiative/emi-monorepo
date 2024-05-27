-- The function `can_update_observations` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_update function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the observations_users_roles table or
-- the observations_teams_users table with an appropriate role id.
CREATE FUNCTION can_update_observations(author_user_id INTEGER, id UUID)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    project_id INTEGER;
    individual_id UUID;
    created_by INTEGER;
    updated_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT project_id, individual_id, created_by, updated_by, 1 INTO project_id, individual_id, created_by, updated_by, canary FROM observations WHERE observations.id = id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN TRUE;
    END IF;
-- If the author_user_id is NULL, we return FALSE.
    IF author_user_id IS NULL THEN
        RETURN FALSE;
    END IF;
-- We check whether the user is the created_by of the row.
    IF author_user_id = created_by THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is the updated_by of the row.
    IF author_user_id = updated_by THEN
        RETURN TRUE;
    END IF;
        IF NOT can_update_projects(author_user_id, project_id) THEN
            RETURN FALSE;
        END IF;
-- If the parent column is not NULL, we call the can_update function of the parent column to determine whether the user can edit the row.
    IF individual_id IS NOT NULL THEN
        IF NOT can_update_sampled_individuals(author_user_id, individual_id) THEN
            RETURN FALSE;
        END IF;
    END IF;
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;

-- The function `can_update_observations_trigger` is a trigger function that checks whether the user can edit the row.
CREATE FUNCTION can_update_observations_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'UPDATE' THEN
        IF NOT can_update_observations(NEW.updated_by, NEW.id) THEN
            RAISE EXCEPTION 'The user does not have the permission to edit this row.';
        END IF;
    END IF;
-- We check whether the user can {operation} the row.
    IF TG_OP = 'INSERT' THEN
        IF NOT can_update_projects(NEW.created_by, NEW.project_id) THEN
            RAISE EXCEPTION 'The user does not have the permission to edit this row.';
        END IF;
    END IF;
-- We check whether the user can {operation} the row.
    IF TG_OP = 'INSERT' AND NEW.individual_id IS NOT NULL THEN
        IF NOT can_update_sampled_individuals(NEW.created_by, NEW.individual_id) THEN
            RAISE EXCEPTION 'The user does not have the permission to edit this row.';
        END IF;
    END IF;
    RETURN NEW;
END;
$$
LANGUAGE plpgsql;

-- We create a trigger that calls the `can_update_observations` function before each INSERT or UPDATE.
CREATE TRIGGER can_update_observations
BEFORE INSERT OR UPDATE ON observations
FOR EACH ROW
EXECUTE FUNCTION can_update_observations_trigger();
-- The function `can_admin_observations` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_delete function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the observations_users_roles table or
-- the observations_teams_users table with an appropriate role id.
CREATE OR REPLACE FUNCTION can_admin_observations(author_user_id INTEGER, id UUID)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    project_id INTEGER;
    individual_id UUID;
    created_by INTEGER;
    updated_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT project_id, individual_id, created_by, updated_by, 1 INTO project_id, individual_id, created_by, updated_by, canary FROM observations WHERE observations.id = id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN TRUE;
    END IF;
-- If the author_user_id is NULL, we return FALSE.
    IF author_user_id IS NULL THEN
        RETURN FALSE;
    END IF;
-- We check whether the user is the created_by of the row.
    IF author_user_id = created_by THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is the updated_by of the row.
    IF author_user_id = updated_by THEN
        RETURN TRUE;
    END IF;
        IF NOT can_delete_projects(author_user_id, project_id) THEN
            RETURN FALSE;
        END IF;
-- If the parent column is not NULL, we call the can_delete function of the parent column to determine whether the user can edit the row.
    IF individual_id IS NOT NULL THEN
        IF NOT can_delete_sampled_individuals(author_user_id, individual_id) THEN
            RETURN FALSE;
        END IF;
    END IF;
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;

-- The function `can_view_observations` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_view function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the observations_users_roles table or
-- the observations_teams_users table with an appropriate role id.
CREATE FUNCTION can_view_observations(author_user_id INTEGER, id UUID)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    project_id INTEGER;
    individual_id UUID;
    created_by INTEGER;
    updated_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT project_id, individual_id, created_by, updated_by, 1 INTO project_id, individual_id, created_by, updated_by, canary FROM observations WHERE observations.id = id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is the created_by of the row.
    IF author_user_id = created_by THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is the updated_by of the row.
    IF author_user_id = updated_by THEN
        RETURN TRUE;
    END IF;
        IF NOT can_view_projects(author_user_id, project_id) THEN
            RETURN FALSE;
        END IF;
-- If the parent column is not NULL, we call the can_view function of the parent column to determine whether the user can edit the row.
    IF individual_id IS NOT NULL THEN
        IF NOT can_view_sampled_individuals(author_user_id, individual_id) THEN
            RETURN FALSE;
        END IF;
    END IF;
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;

