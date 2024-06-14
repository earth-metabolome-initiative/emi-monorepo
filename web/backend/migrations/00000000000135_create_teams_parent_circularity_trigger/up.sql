-- Create the `teams_parent_circularity_trigger` trigger on the teams table.

CREATE FUNCTION teams_parent_circularity_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.parent_team_id = OLD.id THEN
        RAISE EXCEPTION 'Circular reference detected.';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER teams_parent_circularity_trigger
BEFORE UPDATE ON teams
FOR EACH ROW
EXECUTE FUNCTION teams_parent_circularity_trigger();
