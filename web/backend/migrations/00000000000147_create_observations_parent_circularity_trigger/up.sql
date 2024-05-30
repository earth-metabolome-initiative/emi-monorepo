-- Create the `observations_parent_circularity_trigger` trigger on the observations table.

CREATE FUNCTION observations_parent_circularity_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.parent_observation_id = OLD.id THEN
        RAISE EXCEPTION 'Circular reference detected.';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER observations_parent_circularity_trigger
BEFORE UPDATE ON observations
FOR EACH ROW
EXECUTE FUNCTION observations_parent_circularity_trigger();
