-- Create the `container_horizontal_rules_updated_at_trigger` trigger on the container_horizontal_rules table.

CREATE OR REPLACE FUNCTION container_horizontal_rules_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER container_horizontal_rules_updated_at_trigger
BEFORE UPDATE ON container_horizontal_rules
FOR EACH ROW
EXECUTE FUNCTION container_horizontal_rules_updated_at_trigger();
