-- Create the `projects_parent_circularity_trigger` trigger on the projects table.

CREATE FUNCTION projects_parent_circularity_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.parent_project_id = OLD.id THEN
        RAISE EXCEPTION 'Circular reference detected.';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER projects_parent_circularity_trigger
BEFORE UPDATE ON projects
FOR EACH ROW
EXECUTE FUNCTION projects_parent_circularity_trigger();
