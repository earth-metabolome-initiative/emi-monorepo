-- Create the `observations_updated_at_trigger` trigger on the observations table.

CREATE FUNCTION observations_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER observations_updated_at_trigger
BEFORE UPDATE ON observations
FOR EACH ROW
EXECUTE FUNCTION observations_updated_at_trigger();
