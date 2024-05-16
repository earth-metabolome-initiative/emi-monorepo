-- Create the `spectra_collections_teams_roles_updated_at_trigger` trigger on the spectra_collections_teams_roles table.

CREATE OR REPLACE FUNCTION spectra_collections_teams_roles_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER spectra_collections_teams_roles_updated_at_trigger
BEFORE UPDATE ON spectra_collections_teams_roles
FOR EACH ROW
EXECUTE FUNCTION spectra_collections_teams_roles_updated_at_trigger();
