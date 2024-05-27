-- Create the `samples_updated_at_trigger` trigger on the samples table.

CREATE FUNCTION samples_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER samples_updated_at_trigger
BEFORE UPDATE ON samples
FOR EACH ROW
EXECUTE FUNCTION samples_updated_at_trigger();
