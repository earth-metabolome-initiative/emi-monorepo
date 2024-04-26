-- Your SQL goes here
-- A migration to create the sample_taxa table.
-- This is a N to M relationship between samples and taxa.
-- A sample can be associated to no or more taxa, and a taxon can be found in multiple samples.
--
CREATE TABLE sample_bio_ott_taxon_items (
    id UUID PRIMARY KEY,
    created_by INTEGER NOT NULL REFERENCES users(id) ON
    DELETE
        CASCADE,
        sample_id UUID NOT NULL REFERENCES samples(id) ON
    DELETE
        CASCADE,
        taxon_id INTEGER NOT NULL REFERENCES bio_ott_taxon_items(id) ON
    DELETE
        CASCADE,
        UNIQUE (sample_id, taxon_id)
);
