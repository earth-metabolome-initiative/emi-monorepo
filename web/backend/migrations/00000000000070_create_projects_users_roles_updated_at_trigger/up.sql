-- Create the `projects_users_roles_updated_at_trigger` trigger on the projects_users_roles table.

CREATE OR REPLACE FUNCTION projects_users_roles_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER projects_users_roles_updated_at_trigger
BEFORE UPDATE ON projects_users_roles
FOR EACH ROW
EXECUTE FUNCTION projects_users_roles_updated_at_trigger();
