-- Create the `nameplates_updated_at_trigger` trigger on the nameplates table.

CREATE FUNCTION nameplates_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER nameplates_updated_at_trigger
BEFORE UPDATE ON nameplates
FOR EACH ROW
EXECUTE FUNCTION nameplates_updated_at_trigger();
