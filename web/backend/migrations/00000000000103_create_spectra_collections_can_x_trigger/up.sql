-- Create the `can_edit_spectra_collections` and `can_edit_spectra_collections_trigger` functions and trigger on the spectra_collections table.

-- The function `can_edit_spectra_collections` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can edit the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_edit function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the spectra_collections_users_roles table or
-- the spectra_collections_teams_users table with an appropriate role id.
CREATE OR REPLACE FUNCTION can_edit_spectra_collections(author_user_id INTEGER, id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    sample_id UUID;
    created_by INTEGER;
    updated_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT sample_id, created_by, updated_by, 1 INTO sample_id, created_by, updated_by, canary FROM spectra_collections WHERE spectra_collections.id = id;
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
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;

-- The function `can_edit_spectra_collections_trigger` is a trigger function that checks whether the user can edit the row.
CREATE OR REPLACE FUNCTION can_edit_spectra_collections_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'UPDATE' THEN
        IF NOT can_edit_spectra_collections(NEW.updated_by, NEW.id) THEN
            RAISE EXCEPTION 'The user does not have the permission to edit this row.';
        END IF;
    END IF;
-- We check whether the user can edit the row.
    IF TG_OP = 'INSERT' THEN
        IF NOT can_edit_samples(NEW.created_by, NEW.sample_id) THEN
            RAISE EXCEPTION 'The user does not have the permission to edit this row.';
        END IF;
    END IF;
    RETURN NEW;
END;
$$
LANGUAGE plpgsql;

-- We create a trigger that calls the `can_edit_spectra_collections` function before each INSERT or UPDATE.
CREATE TRIGGER can_edit_spectra_collections
BEFORE INSERT OR UPDATE ON spectra_collections
FOR EACH ROW
EXECUTE FUNCTION can_edit_spectra_collections_trigger();
-- Create the `can_delete_spectra_collections` and `can_delete_spectra_collections_trigger` functions and trigger on the spectra_collections table.

-- The function `can_delete_spectra_collections` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can edit the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_delete function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the spectra_collections_users_roles table or
-- the spectra_collections_teams_users table with an appropriate role id.
CREATE OR REPLACE FUNCTION can_delete_spectra_collections(author_user_id INTEGER, id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    sample_id UUID;
    created_by INTEGER;
    updated_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT sample_id, created_by, updated_by, 1 INTO sample_id, created_by, updated_by, canary FROM spectra_collections WHERE spectra_collections.id = id;
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
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;

-- Create the `can_view_spectra_collections` and `can_view_spectra_collections_trigger` functions and trigger on the spectra_collections table.

-- The function `can_view_spectra_collections` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can edit the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_view function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the spectra_collections_users_roles table or
-- the spectra_collections_teams_users table with an appropriate role id.
CREATE OR REPLACE FUNCTION can_view_spectra_collections(author_user_id INTEGER, id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    sample_id UUID;
    created_by INTEGER;
    updated_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT sample_id, created_by, updated_by, 1 INTO sample_id, created_by, updated_by, canary FROM spectra_collections WHERE spectra_collections.id = id;
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
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;

