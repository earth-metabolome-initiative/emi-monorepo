-- Create the `teams_updated_at_trigger` trigger on the teams table.

CREATE FUNCTION teams_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER teams_updated_at_trigger
BEFORE UPDATE ON teams
FOR EACH ROW
EXECUTE FUNCTION teams_updated_at_trigger();
