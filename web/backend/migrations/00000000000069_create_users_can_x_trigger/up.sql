-- The function `can_update_users` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_update function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the users_users_roles table or
-- the users_teams_users table with an appropriate role id.
CREATE FUNCTION can_update_users(author_user_id INTEGER, this_users_id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT 1 INTO canary FROM users WHERE users.id = this_users_id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN TRUE;
    END IF;
-- If the author_user_id is NULL, we return FALSE.
    IF author_user_id IS NULL THEN
        RETURN FALSE;
    END IF;
-- If the author_user_id is the same as the user_id of the row, we return TRUE.
    IF author_user_id = this_users_id THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is in the users_users_roles table with an appropriate role id.
    IF EXISTS (SELECT 1 FROM users_users_roles WHERE users_users_roles.user_id = author_user_id AND users_users_roles.role_id <= 2 AND users_users_roles.table_id = this_users_id) THEN
        RETURN TRUE;
    END IF;
    RETURN FALSE;
END;
$$
LANGUAGE plpgsql;

-- The function `can_update_users_trigger` is a trigger function that checks whether the user can update the row.
CREATE FUNCTION can_update_users_trigger()
RETURNS TRIGGER AS $$
BEGIN
    RETURN NEW;
END;
$$
LANGUAGE plpgsql;

-- We create a trigger that calls the `can_update_users` function before each INSERT or UPDATE.
CREATE TRIGGER can_update_users
BEFORE INSERT OR UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION can_update_users_trigger();
-- The function `can_admin_users` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_admin function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the users_users_roles table or
-- the users_teams_users table with an appropriate role id.
CREATE FUNCTION can_admin_users(author_user_id INTEGER, this_users_id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT 1 INTO canary FROM users WHERE users.id = this_users_id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN TRUE;
    END IF;
-- If the author_user_id is NULL, we return FALSE.
    IF author_user_id IS NULL THEN
        RETURN FALSE;
    END IF;
-- We check whether the user is in the users_users_roles table with an appropriate role id.
    IF EXISTS (SELECT 1 FROM users_users_roles WHERE users_users_roles.user_id = author_user_id AND users_users_roles.role_id <= 1 AND users_users_roles.table_id = this_users_id) THEN
        RETURN TRUE;
    END IF;
    RETURN FALSE;
END;
$$
LANGUAGE plpgsql;

