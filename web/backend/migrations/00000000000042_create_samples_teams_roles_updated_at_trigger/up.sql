-- Create the `samples_teams_roles_updated_at_trigger` trigger on the samples_teams_roles table.

CREATE OR REPLACE FUNCTION samples_teams_roles_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER samples_teams_roles_updated_at_trigger
BEFORE UPDATE ON samples_teams_roles
FOR EACH ROW
EXECUTE FUNCTION samples_teams_roles_updated_at_trigger();
