-- The function `can_edit_users` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can edit the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_edit function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the users_users_roles table or
-- the users_teams_users table with an appropriate role id.
CREATE OR REPLACE FUNCTION can_edit_users(author_user_id INTEGER, id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    id INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT id, 1 INTO id, canary FROM users WHERE users.id = id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is the id of the row.
    IF author_user_id = id THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is in the users_users_roles table with an appropriate role id.
    IF EXISTS (SELECT 1 FROM users_users_roles WHERE users_users_roles.user_id = author_user_id AND users_users_roles.role_id <= 2 AND users_users_roles.table_id == id) THEN
        RETURN TRUE;
    END IF;
    RETURN FALSE;
END;
$$
LANGUAGE plpgsql;

-- The function `can_edit_users_trigger` is a trigger function that checks whether the user can edit the row.
CREATE OR REPLACE FUNCTION can_edit_users_trigger()
RETURNS TRIGGER AS $$
BEGIN
    RETURN NEW;
END;
$$
LANGUAGE plpgsql;

-- We create a trigger that calls the `can_edit_users` function before each INSERT or UPDATE.
CREATE TRIGGER can_edit_users
BEFORE INSERT OR UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION can_edit_users_trigger();
-- The function `can_delete_users` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can edit the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_delete function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the users_users_roles table or
-- the users_teams_users table with an appropriate role id.
CREATE OR REPLACE FUNCTION can_delete_users(author_user_id INTEGER, id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    id INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT id, 1 INTO id, canary FROM users WHERE users.id = id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is the id of the row.
    IF author_user_id = id THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is in the users_users_roles table with an appropriate role id.
    IF EXISTS (SELECT 1 FROM users_users_roles WHERE users_users_roles.user_id = author_user_id AND users_users_roles.role_id <= 1 AND users_users_roles.table_id == id) THEN
        RETURN TRUE;
    END IF;
    RETURN FALSE;
END;
$$
LANGUAGE plpgsql;

-- The function `can_view_users` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can edit the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_view function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the users_users_roles table or
-- the users_teams_users table with an appropriate role id.
CREATE OR REPLACE FUNCTION can_view_users(author_user_id INTEGER, id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    id INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT id, 1 INTO id, canary FROM users WHERE users.id = id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is the id of the row.
    IF author_user_id = id THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is in the users_users_roles table with an appropriate role id.
    IF EXISTS (SELECT 1 FROM users_users_roles WHERE users_users_roles.user_id = author_user_id AND users_users_roles.role_id <= 3 AND users_users_roles.table_id == id) THEN
        RETURN TRUE;
    END IF;
    RETURN FALSE;
END;
$$
LANGUAGE plpgsql;

