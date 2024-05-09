-- Create the `project_requirements_updated_at_trigger` trigger on the project_requirements table.

CREATE OR REPLACE FUNCTION project_requirements_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER project_requirements_updated_at_trigger
BEFORE UPDATE ON project_requirements
FOR EACH ROW
EXECUTE FUNCTION project_requirements_updated_at_trigger();
