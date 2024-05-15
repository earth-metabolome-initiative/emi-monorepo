-- Create the `sample_bio_ott_taxon_items_updated_at_trigger` trigger on the sample_bio_ott_taxon_items table.

CREATE OR REPLACE FUNCTION sample_bio_ott_taxon_items_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER sample_bio_ott_taxon_items_updated_at_trigger
BEFORE UPDATE ON sample_bio_ott_taxon_items
FOR EACH ROW
EXECUTE FUNCTION sample_bio_ott_taxon_items_updated_at_trigger();
