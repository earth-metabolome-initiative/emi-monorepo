-- The function `can_update_sample_bio_ott_taxon_items` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_update function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the sample_bio_ott_taxon_items_users_roles table or
-- the sample_bio_ott_taxon_items_teams_users table with an appropriate role id.
CREATE FUNCTION can_update_sample_bio_ott_taxon_items(author_user_id INTEGER, sample_id UUID, taxon_id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    sample_id UUID;
    created_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT sample_id, created_by, 1 INTO sample_id, created_by, canary FROM sample_bio_ott_taxon_items WHERE sample_bio_ott_taxon_items.sample_id = sample_id AND sample_bio_ott_taxon_items.taxon_id = taxon_id;
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
        IF NOT can_update_samples(author_user_id, sample_id) THEN
            RETURN FALSE;
        END IF;
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;

-- The function `can_admin_sample_bio_ott_taxon_items` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_admin function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the sample_bio_ott_taxon_items_users_roles table or
-- the sample_bio_ott_taxon_items_teams_users table with an appropriate role id.
CREATE FUNCTION can_admin_sample_bio_ott_taxon_items(author_user_id INTEGER, sample_id UUID, taxon_id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    sample_id UUID;
    created_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT sample_id, created_by, 1 INTO sample_id, created_by, canary FROM sample_bio_ott_taxon_items WHERE sample_bio_ott_taxon_items.sample_id = sample_id AND sample_bio_ott_taxon_items.taxon_id = taxon_id;
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
        IF NOT can_admin_samples(author_user_id, sample_id) THEN
            RETURN FALSE;
        END IF;
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;

-- The function `can_view_sample_bio_ott_taxon_items` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_view function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the sample_bio_ott_taxon_items_users_roles table or
-- the sample_bio_ott_taxon_items_teams_users table with an appropriate role id.
CREATE FUNCTION can_view_sample_bio_ott_taxon_items(author_user_id INTEGER, sample_id UUID, taxon_id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    sample_id UUID;
    created_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT sample_id, created_by, 1 INTO sample_id, created_by, canary FROM sample_bio_ott_taxon_items WHERE sample_bio_ott_taxon_items.sample_id = sample_id AND sample_bio_ott_taxon_items.taxon_id = taxon_id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is the created_by of the row.
    IF author_user_id = created_by THEN
        RETURN TRUE;
    END IF;
        IF NOT can_view_samples(author_user_id, sample_id) THEN
            RETURN FALSE;
        END IF;
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;

