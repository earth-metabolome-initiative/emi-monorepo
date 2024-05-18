-- Your SQL goes here
-- A migration to create the sample_taxa table.
-- This is a N to M relationship between samples and taxa.
-- A sample can be associated to no or more taxa, and a taxon can be found in multiple samples.
--
CREATE TABLE IF NOT EXISTS sample_bio_ott_taxon_items (
    created_by INTEGER NOT NULL REFERENCES users(id) ON
    DELETE
        CASCADE,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_by INTEGER NOT NULL REFERENCES users(id) ON
    DELETE
        CASCADE,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        sample_id UUID NOT NULL REFERENCES samples(id) ON
    DELETE
        CASCADE,
        taxon_id INTEGER NOT NULL REFERENCES bio_ott_taxon_items(id) ON
    DELETE
        CASCADE,
    PRIMARY KEY (sample_id, taxon_id)
);