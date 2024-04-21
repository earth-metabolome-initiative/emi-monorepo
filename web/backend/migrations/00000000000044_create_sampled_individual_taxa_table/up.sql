-- Your SQL goes here
-- A migration to create the sampled_individual_taxa table.
-- This is a N to M relationship between sampled_individuals and taxa.
-- An individual can be associated to no or more taxa, and a taxon can be found in multiple individuals.
--
CREATE TABLE sampled_individual_taxa (
  id UUID PRIMARY KEY,
  created_by INTEGER NOT NULL REFERENCES users(id) ON
  DELETE
    CASCADE,
    sampled_individual_id UUID NOT NULL REFERENCES sampled_individuals(id) ON
  DELETE
    CASCADE,
    taxon_id INTEGER NOT NULL REFERENCES taxa(id) ON
  DELETE
    CASCADE,
    UNIQUE (sampled_individual_id, taxon_id)
);