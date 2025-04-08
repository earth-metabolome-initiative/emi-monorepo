-- Your SQL goes here
-- A migration to create the organism_taxa table.
-- This is a N to M relationship between organisms and taxa.
-- An organism can be associated to no or more taxa, and a taxon can be found in multiple organisms.
--
CREATE TABLE IF NOT EXISTS organism_taxa (
  created_by INTEGER NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  organism_id UUID NOT NULL,
  taxon_id INTEGER NOT NULL,
  PRIMARY KEY (organism_id, taxon_id),
  FOREIGN KEY (created_by) REFERENCES users(id),
  FOREIGN KEY (organism_id) REFERENCES organisms(id) ON DELETE CASCADE,
  FOREIGN KEY (taxon_id) REFERENCES taxa(id)
);