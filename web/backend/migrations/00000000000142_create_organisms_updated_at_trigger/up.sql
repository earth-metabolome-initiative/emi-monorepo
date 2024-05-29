-- Create the `organisms_updated_at_trigger` trigger on the organisms table.

CREATE FUNCTION organisms_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER organisms_updated_at_trigger
BEFORE UPDATE ON organisms
FOR EACH ROW
EXECUTE FUNCTION organisms_updated_at_trigger();
