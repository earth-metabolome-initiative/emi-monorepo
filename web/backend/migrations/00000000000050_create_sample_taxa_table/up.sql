-- Your SQL goes here
-- A migration to create the sample_taxa table.
-- This is a N to M relationship between samples and taxa.
-- A sample can be associated to no or more taxa, and a taxon can be found in multiple samples.
--
CREATE TABLE sample_taxa (
    id UUID PRIMARY KEY REFERENCES editables (id) ON DELETE CASCADE,
  sample_id UUID NOT NULL REFERENCES samples(id) ON DELETE CASCADE,
  taxon_id UUID NOT NULL REFERENCES taxa(id) ON DELETE CASCADE,
  UNIQUE (sample_id, taxon_id)
);

-- Add a trigger to delete the corresponding record in the editables table when a sample_taxa is deleted.
CREATE OR REPLACE FUNCTION delete_editables() RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM
        editables
    WHERE
        id = OLD.sample_id;

    RETURN OLD;

END;

$$ LANGUAGE plpgsql;

CREATE TRIGGER delete_editables AFTER
DELETE
    ON sample_taxa FOR EACH ROW EXECUTE FUNCTION delete_editables();
