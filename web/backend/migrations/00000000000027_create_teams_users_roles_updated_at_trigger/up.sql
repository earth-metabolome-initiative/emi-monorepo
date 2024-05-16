-- Create the `teams_users_roles_updated_at_trigger` trigger on the teams_users_roles table.

CREATE OR REPLACE FUNCTION teams_users_roles_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER teams_users_roles_updated_at_trigger
BEFORE UPDATE ON teams_users_roles
FOR EACH ROW
EXECUTE FUNCTION teams_users_roles_updated_at_trigger();
