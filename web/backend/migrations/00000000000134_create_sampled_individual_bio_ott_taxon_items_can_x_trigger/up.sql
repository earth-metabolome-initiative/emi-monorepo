-- The function `can_update_sampled_individual_bio_ott_taxon_items` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_update function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the sampled_individual_bio_ott_taxon_items_users_roles table or
-- the sampled_individual_bio_ott_taxon_items_teams_users table with an appropriate role id.
CREATE FUNCTION can_update_sampled_individual_bio_ott_taxon_items(author_user_id INTEGER, this_sampled_individual_bio_ott_taxon_items_sampled_individual_id UUID, this_sampled_individual_bio_ott_taxon_items_taxon_id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    this_sampled_individual_id UUID;
    this_created_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT sampled_individual_id, created_by, 1 INTO this_sampled_individual_id, this_created_by, canary FROM sampled_individual_bio_ott_taxon_items WHERE sampled_individual_bio_ott_taxon_items.sampled_individual_id = this_sampled_individual_bio_ott_taxon_items_sampled_individual_id AND sampled_individual_bio_ott_taxon_items.taxon_id = this_sampled_individual_bio_ott_taxon_items_taxon_id;
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
        IF NOT can_update_sampled_individuals(author_user_id, this_sampled_individual_id) THEN
            RETURN FALSE;
        END IF;
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;

-- The function `can_admin_sampled_individual_bio_ott_taxon_items` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_delete function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the sampled_individual_bio_ott_taxon_items_users_roles table or
-- the sampled_individual_bio_ott_taxon_items_teams_users table with an appropriate role id.
CREATE FUNCTION can_admin_sampled_individual_bio_ott_taxon_items(author_user_id INTEGER, this_sampled_individual_bio_ott_taxon_items_sampled_individual_id UUID, this_sampled_individual_bio_ott_taxon_items_taxon_id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    this_sampled_individual_id UUID;
    this_created_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT sampled_individual_id, created_by, 1 INTO this_sampled_individual_id, this_created_by, canary FROM sampled_individual_bio_ott_taxon_items WHERE sampled_individual_bio_ott_taxon_items.sampled_individual_id = this_sampled_individual_bio_ott_taxon_items_sampled_individual_id AND sampled_individual_bio_ott_taxon_items.taxon_id = this_sampled_individual_bio_ott_taxon_items_taxon_id;
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
        IF NOT can_delete_sampled_individuals(author_user_id, this_sampled_individual_id) THEN
            RETURN FALSE;
        END IF;
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;

-- The function `can_view_sampled_individual_bio_ott_taxon_items` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_view function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the sampled_individual_bio_ott_taxon_items_users_roles table or
-- the sampled_individual_bio_ott_taxon_items_teams_users table with an appropriate role id.
CREATE FUNCTION can_view_sampled_individual_bio_ott_taxon_items(author_user_id INTEGER, this_sampled_individual_bio_ott_taxon_items_sampled_individual_id UUID, this_sampled_individual_bio_ott_taxon_items_taxon_id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    this_sampled_individual_id UUID;
    this_created_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT sampled_individual_id, created_by, 1 INTO this_sampled_individual_id, this_created_by, canary FROM sampled_individual_bio_ott_taxon_items WHERE sampled_individual_bio_ott_taxon_items.sampled_individual_id = this_sampled_individual_bio_ott_taxon_items_sampled_individual_id AND sampled_individual_bio_ott_taxon_items.taxon_id = this_sampled_individual_bio_ott_taxon_items_taxon_id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is the created_by of the row.
    IF author_user_id = this_created_by THEN
        RETURN TRUE;
    END IF;
        IF NOT can_view_sampled_individuals(author_user_id, this_sampled_individual_id) THEN
            RETURN FALSE;
        END IF;
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;

