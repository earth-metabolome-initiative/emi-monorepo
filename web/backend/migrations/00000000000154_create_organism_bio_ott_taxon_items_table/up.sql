-- Your SQL goes here
-- A migration to create the organism_taxa table.
-- This is a N to M relationship between organisms and taxa.
-- An organism can be associated to no or more taxa, and a taxon can be found in multiple organisms.
--
CREATE TABLE IF NOT EXISTS organism_bio_ott_taxon_items (
  created_by INTEGER NOT NULL REFERENCES users(id) ON
  DELETE
    CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    organism_id UUID NOT NULL REFERENCES organisms(id) ON
  DELETE
    CASCADE,
    taxon_id INTEGER NOT NULL REFERENCES bio_ott_taxon_items(id) ON
  DELETE
    CASCADE,
    PRIMARY KEY (organism_id, taxon_id)
);