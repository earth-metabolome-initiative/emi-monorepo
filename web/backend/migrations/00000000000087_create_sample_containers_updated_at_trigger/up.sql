-- Create the `sample_containers_updated_at_trigger` trigger on the sample_containers table.

CREATE FUNCTION sample_containers_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER sample_containers_updated_at_trigger
BEFORE UPDATE ON sample_containers
FOR EACH ROW
EXECUTE FUNCTION sample_containers_updated_at_trigger();
