-- Create the `sampling_procedures_updated_at_trigger` trigger on the sampling_procedures table.

CREATE OR REPLACE FUNCTION sampling_procedures_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER sampling_procedures_updated_at_trigger
BEFORE UPDATE ON sampling_procedures
FOR EACH ROW
EXECUTE FUNCTION sampling_procedures_updated_at_trigger();
