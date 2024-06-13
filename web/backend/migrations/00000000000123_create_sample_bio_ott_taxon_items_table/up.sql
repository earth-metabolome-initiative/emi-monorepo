-- Your SQL goes here
-- A migration to create the sample_taxa table.
-- This is a N to M relationship between samples and taxa.
-- A sample can be associated to no or more taxa, and a taxon can be found in multiple samples.
--
CREATE TABLE IF NOT EXISTS sample_bio_ott_taxon_items (
    created_by INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    sample_id UUID NOT NULL,
    taxon_id INTEGER NOT NULL,
    PRIMARY KEY (sample_id, taxon_id),
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (sample_id) REFERENCES samples(id) ON DELETE CASCADE,
    FOREIGN KEY (taxon_id) REFERENCES bio_ott_taxon_items(id) ON DELETE CASCADE
);