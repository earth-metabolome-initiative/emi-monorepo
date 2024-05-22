-- Create the `bio_ott_taxon_items_parent_circularity_trigger` trigger on the bio_ott_taxon_items table.

CREATE OR REPLACE FUNCTION bio_ott_taxon_items_parent_circularity_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.domain_id = OLD.id THEN
        RAISE EXCEPTION 'Circular reference detected.';
    END IF;
    IF NEW.kingdom_id = OLD.id THEN
        RAISE EXCEPTION 'Circular reference detected.';
    END IF;
    IF NEW.phylum_id = OLD.id THEN
        RAISE EXCEPTION 'Circular reference detected.';
    END IF;
    IF NEW.class_id = OLD.id THEN
        RAISE EXCEPTION 'Circular reference detected.';
    END IF;
    IF NEW.order_id = OLD.id THEN
        RAISE EXCEPTION 'Circular reference detected.';
    END IF;
    IF NEW.family_id = OLD.id THEN
        RAISE EXCEPTION 'Circular reference detected.';
    END IF;
    IF NEW.genus_id = OLD.id THEN
        RAISE EXCEPTION 'Circular reference detected.';
    END IF;
    IF NEW.parent_id = OLD.id THEN
        RAISE EXCEPTION 'Circular reference detected.';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER bio_ott_taxon_items_parent_circularity_trigger
BEFORE UPDATE ON bio_ott_taxon_items
FOR EACH ROW
EXECUTE FUNCTION bio_ott_taxon_items_parent_circularity_trigger();
