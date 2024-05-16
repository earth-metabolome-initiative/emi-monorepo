-- Create the `sampled_individuals_users_roles_updated_at_trigger` trigger on the sampled_individuals_users_roles table.

CREATE OR REPLACE FUNCTION sampled_individuals_users_roles_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER sampled_individuals_users_roles_updated_at_trigger
BEFORE UPDATE ON sampled_individuals_users_roles
FOR EACH ROW
EXECUTE FUNCTION sampled_individuals_users_roles_updated_at_trigger();
