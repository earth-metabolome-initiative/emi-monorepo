-- Create the `can_edit_sampled_individual_bio_ott_taxon_items` and `can_edit_sampled_individual_bio_ott_taxon_items_trigger` functions and trigger on the sampled_individual_bio_ott_taxon_items table.

-- The function `can_edit_sampled_individual_bio_ott_taxon_items` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can edit the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_edit function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the sampled_individual_bio_ott_taxon_items_users_roles table or
-- the sampled_individual_bio_ott_taxon_items_teams_users table with an appropriate role id.
CREATE OR REPLACE FUNCTION can_edit_sampled_individual_bio_ott_taxon_items(author_user_id INTEGER, sampled_individual_id UUID, taxon_id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    sampled_individual_id UUID;
    created_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT sampled_individual_id, created_by, 1 INTO sampled_individual_id, created_by, canary FROM sampled_individual_bio_ott_taxon_items WHERE sampled_individual_bio_ott_taxon_items.sampled_individual_id = sampled_individual_id AND sampled_individual_bio_ott_taxon_items.taxon_id = taxon_id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN TRUE;
    END IF;
        IF NOT can_edit_sampled_individuals(author_user_id, sampled_individual_id) THEN
            RETURN FALSE;
        END IF;
-- We check whether the user is the created_by of the row.
    IF author_user_id = created_by THEN
        RETURN TRUE;
    END IF;
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;

-- The function `can_edit_sampled_individual_bio_ott_taxon_items_trigger` is a trigger function that checks whether the user can edit the row.
CREATE OR REPLACE FUNCTION can_edit_sampled_individual_bio_ott_taxon_items_trigger()
RETURNS TRIGGER AS $$
BEGIN
-- We check whether the user can edit the row.
    IF TG_OP = 'INSERT' THEN
        IF NOT can_edit_sampled_individuals(NEW.created_by, NEW.sampled_individual_id) THEN
            RAISE EXCEPTION 'The user does not have the permission to edit this row.';
        END IF;
    END IF;
    RETURN NEW;
END;
$$
LANGUAGE plpgsql;

-- We create a trigger that calls the `can_edit_sampled_individual_bio_ott_taxon_items` function before each INSERT or UPDATE.
CREATE TRIGGER can_edit_sampled_individual_bio_ott_taxon_items
BEFORE INSERT OR UPDATE ON sampled_individual_bio_ott_taxon_items
FOR EACH ROW
EXECUTE FUNCTION can_edit_sampled_individual_bio_ott_taxon_items_trigger();
-- Create the `can_delete_sampled_individual_bio_ott_taxon_items` and `can_delete_sampled_individual_bio_ott_taxon_items_trigger` functions and trigger on the sampled_individual_bio_ott_taxon_items table.

-- The function `can_delete_sampled_individual_bio_ott_taxon_items` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can edit the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_delete function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the sampled_individual_bio_ott_taxon_items_users_roles table or
-- the sampled_individual_bio_ott_taxon_items_teams_users table with an appropriate role id.
CREATE OR REPLACE FUNCTION can_delete_sampled_individual_bio_ott_taxon_items(author_user_id INTEGER, sampled_individual_id UUID, taxon_id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    sampled_individual_id UUID;
    created_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT sampled_individual_id, created_by, 1 INTO sampled_individual_id, created_by, canary FROM sampled_individual_bio_ott_taxon_items WHERE sampled_individual_bio_ott_taxon_items.sampled_individual_id = sampled_individual_id AND sampled_individual_bio_ott_taxon_items.taxon_id = taxon_id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN TRUE;
    END IF;
        IF NOT can_delete_sampled_individuals(author_user_id, sampled_individual_id) THEN
            RETURN FALSE;
        END IF;
-- We check whether the user is the created_by of the row.
    IF author_user_id = created_by THEN
        RETURN TRUE;
    END IF;
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;

-- Create the `can_view_sampled_individual_bio_ott_taxon_items` and `can_view_sampled_individual_bio_ott_taxon_items_trigger` functions and trigger on the sampled_individual_bio_ott_taxon_items table.

-- The function `can_view_sampled_individual_bio_ott_taxon_items` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can edit the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_view function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the sampled_individual_bio_ott_taxon_items_users_roles table or
-- the sampled_individual_bio_ott_taxon_items_teams_users table with an appropriate role id.
CREATE OR REPLACE FUNCTION can_view_sampled_individual_bio_ott_taxon_items(author_user_id INTEGER, sampled_individual_id UUID, taxon_id INTEGER)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    sampled_individual_id UUID;
    created_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT sampled_individual_id, created_by, 1 INTO sampled_individual_id, created_by, canary FROM sampled_individual_bio_ott_taxon_items WHERE sampled_individual_bio_ott_taxon_items.sampled_individual_id = sampled_individual_id AND sampled_individual_bio_ott_taxon_items.taxon_id = taxon_id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN TRUE;
    END IF;
        IF NOT can_view_sampled_individuals(author_user_id, sampled_individual_id) THEN
            RETURN FALSE;
        END IF;
-- We check whether the user is the created_by of the row.
    IF author_user_id = created_by THEN
        RETURN TRUE;
    END IF;
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql;
