-- Your SQL goes here
-- A migration to create the sampled_individual_taxa table.
-- This is a N to M relationship between sampled_individuals and taxa.
-- An individual can be associated to no or more taxa, and a taxon can be found in multiple individuals.
--
CREATE TABLE sampled_individual_taxa (
  sampled_individual_id UUID NOT NULL REFERENCES sampled_individuals(id) ON DELETE CASCADE,
  taxon_id UUID NOT NULL REFERENCES taxa(id) ON DELETE CASCADE,
  PRIMARY KEY (sampled_individual_id, taxon_id)
);

-- Add a trigger to delete the corresponding record in the editables table when a sampled_individual_taxa is deleted.
CREATE OR REPLACE FUNCTION delete_editables() RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM
        editables
    WHERE
        id = OLD.sampled_individual_id;

    RETURN OLD;

END;

$$ LANGUAGE plpgsql;

CREATE TRIGGER delete_editables AFTER
DELETE
    ON sampled_individual_taxa FOR EACH ROW EXECUTE FUNCTION delete_editables();
