-- Create the `item_categories_updated_at_trigger` trigger on the item_categories table.

CREATE OR REPLACE FUNCTION item_categories_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER item_categories_updated_at_trigger
BEFORE UPDATE ON item_categories
FOR EACH ROW
EXECUTE FUNCTION item_categories_updated_at_trigger();
