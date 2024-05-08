-- Create the `procedures_updated_at_trigger` trigger on the procedures table.

CREATE OR REPLACE FUNCTION procedures_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER procedures_updated_at_trigger
BEFORE UPDATE ON procedures
FOR EACH ROW
EXECUTE FUNCTION procedures_updated_at_trigger();
