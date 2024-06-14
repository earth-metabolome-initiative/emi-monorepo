-- Create the `derived_samples_updated_at_trigger` trigger on the derived_samples table.

CREATE OR REPLACE FUNCTION derived_samples_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER derived_samples_updated_at_trigger
BEFORE UPDATE ON derived_samples
FOR EACH ROW
EXECUTE FUNCTION derived_samples_updated_at_trigger();
