-- Your SQL goes here
-- A migration to create the sample_taxa table.
-- This is a N to M relationship between samples and taxa.
-- A sample can be associated to no or more taxa, and a taxon can be found in multiple samples.
--
CREATE TABLE sample_taxa (
  sample_id INTEGER NOT NULL REFERENCES samples(id) ON DELETE CASCADE,
  taxon_id INTEGER NOT NULL REFERENCES taxa(id) ON DELETE CASCADE,
  PRIMARY KEY (sample_id, taxon_id)
);

