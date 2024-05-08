-- Create the `container_vertical_rules_updated_at_trigger` trigger on the container_vertical_rules table.

CREATE OR REPLACE FUNCTION container_vertical_rules_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER container_vertical_rules_updated_at_trigger
BEFORE UPDATE ON container_vertical_rules
FOR EACH ROW
EXECUTE FUNCTION container_vertical_rules_updated_at_trigger();
