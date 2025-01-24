-- Create the `organisms_parent_circularity_trigger` trigger on the organisms table.

CREATE FUNCTION organisms_parent_circularity_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.parent_organism_id = OLD.id THEN
        RAISE EXCEPTION 'Circular reference detected.';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER organisms_parent_circularity_trigger
BEFORE UPDATE ON organisms
FOR EACH ROW
EXECUTE FUNCTION organisms_parent_circularity_trigger();
