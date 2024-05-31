-- The function `can_update_derived_samples` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_update function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the derived_samples_users_roles table or
-- the derived_samples_teams_users table with an appropriate role id.
CREATE FUNCTION can_update_derived_samples(author_user_id INTEGER, this_derived_samples_parent_sample_id UUID, this_derived_samples_child_sample_id UUID)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    this_parent_sample_id UUID;
    this_child_sample_id UUID;
    this_created_by INTEGER;
    this_updated_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT parent_sample_id, child_sample_id, created_by, updated_by, 1 INTO this_parent_sample_id, this_child_sample_id, this_created_by, this_updated_by, canary FROM derived_samples WHERE derived_samples.parent_sample_id = this_derived_samples_parent_sample_id AND derived_samples.child_sample_id = this_derived_samples_child_sample_id;
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
        IF NOT can_update_samples(author_user_id, this_parent_sample_id) THEN
            RETURN FALSE;
        END IF;
        IF NOT can_update_samples(author_user_id, this_child_sample_id) THEN
            RETURN FALSE;
        END IF;
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;

-- The function `can_update_derived_samples_trigger` is a trigger function that checks whether the user can update the row.
CREATE FUNCTION can_update_derived_samples_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'UPDATE' THEN
        IF NOT can_update_derived_samples(NEW.updated_by, ( NEW.parent_sample_id, NEW.child_sample_id )) THEN
            RAISE EXCEPTION 'The user does not have the permission to update this row.';
        END IF;
    END IF;
-- We check whether the user can update the row.
    IF TG_OP = 'INSERT' THEN
        IF NOT can_update_samples(NEW.created_by, NEW.parent_sample_id) THEN
            RAISE EXCEPTION 'The user does not have the permission to update this row.';
        END IF;
    END IF;
-- We check whether the user can update the row.
    IF TG_OP = 'INSERT' THEN
        IF NOT can_update_samples(NEW.created_by, NEW.child_sample_id) THEN
            RAISE EXCEPTION 'The user does not have the permission to update this row.';
        END IF;
    END IF;
    RETURN NEW;
END;
$$
LANGUAGE plpgsql;

-- We create a trigger that calls the `can_update_derived_samples` function before each INSERT or UPDATE.
CREATE TRIGGER can_update_derived_samples
BEFORE INSERT OR UPDATE ON derived_samples
FOR EACH ROW
EXECUTE FUNCTION can_update_derived_samples_trigger();
-- The function `can_admin_derived_samples` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_delete function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the derived_samples_users_roles table or
-- the derived_samples_teams_users table with an appropriate role id.
CREATE FUNCTION can_admin_derived_samples(author_user_id INTEGER, this_derived_samples_parent_sample_id UUID, this_derived_samples_child_sample_id UUID)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    this_parent_sample_id UUID;
    this_child_sample_id UUID;
    this_created_by INTEGER;
    this_updated_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT parent_sample_id, child_sample_id, created_by, updated_by, 1 INTO this_parent_sample_id, this_child_sample_id, this_created_by, this_updated_by, canary FROM derived_samples WHERE derived_samples.parent_sample_id = this_derived_samples_parent_sample_id AND derived_samples.child_sample_id = this_derived_samples_child_sample_id;
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
        IF NOT can_delete_samples(author_user_id, this_parent_sample_id) THEN
            RETURN FALSE;
        END IF;
        IF NOT can_delete_samples(author_user_id, this_child_sample_id) THEN
            RETURN FALSE;
        END IF;
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;

-- The function `can_view_derived_samples` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_view function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the derived_samples_users_roles table or
-- the derived_samples_teams_users table with an appropriate role id.
CREATE FUNCTION can_view_derived_samples(author_user_id INTEGER, this_derived_samples_parent_sample_id UUID, this_derived_samples_child_sample_id UUID)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    this_parent_sample_id UUID;
    this_child_sample_id UUID;
    this_created_by INTEGER;
    this_updated_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT parent_sample_id, child_sample_id, created_by, updated_by, 1 INTO this_parent_sample_id, this_child_sample_id, this_created_by, this_updated_by, canary FROM derived_samples WHERE derived_samples.parent_sample_id = this_derived_samples_parent_sample_id AND derived_samples.child_sample_id = this_derived_samples_child_sample_id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
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
        IF NOT can_view_samples(author_user_id, this_parent_sample_id) THEN
            RETURN FALSE;
        END IF;
        IF NOT can_view_samples(author_user_id, this_child_sample_id) THEN
            RETURN FALSE;
        END IF;
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;
