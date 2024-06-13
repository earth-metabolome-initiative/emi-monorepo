-- The function `can_view_spectra` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_view function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the spectra_users_roles table or
-- the spectra_teams_users table with an appropriate role id.
CREATE FUNCTION can_view_spectra(author_user_id INTEGER, this_spectra_id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    this_spectra_collection_id INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT spectra_collection_id, 1 INTO this_spectra_collection_id, canary FROM spectra WHERE spectra.id = this_spectra_id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN TRUE;
    END IF;
        IF NOT can_view_spectra_collections(author_user_id, this_spectra_collection_id) THEN
            RETURN FALSE;
        END IF;
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;

