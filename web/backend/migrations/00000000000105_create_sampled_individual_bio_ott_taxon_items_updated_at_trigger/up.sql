-- Create the `sampled_individual_bio_ott_taxon_items_updated_at_trigger` trigger on the sampled_individual_bio_ott_taxon_items table.

CREATE OR REPLACE FUNCTION sampled_individual_bio_ott_taxon_items_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER sampled_individual_bio_ott_taxon_items_updated_at_trigger
BEFORE UPDATE ON sampled_individual_bio_ott_taxon_items
FOR EACH ROW
EXECUTE FUNCTION sampled_individual_bio_ott_taxon_items_updated_at_trigger();
