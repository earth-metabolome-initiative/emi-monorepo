-- Your SQL goes here
CREATE TABLE IF NOT EXISTS bio_order_items (
    -- The unique identifier for the taxon
    id INTEGER PRIMARY KEY,
    -- The scientific name of the taxon
    name TEXT NOT NULL,
    -- The NCBI Taxon ID is a unique identifier for a taxon in the NCBI Taxonomy database
    -- which may be NULL when this taxon is not present in the NCBI Taxonomy database.
    ott_id INTEGER NOT NULL, -- cannot be null
    ncbi_id INTEGER, -- because it can be null
    gbif_id INTEGER, -- because it can be null
    bio_domain_id INTEGER REFERENCES bio_domain_items(id) ON DELETE CASCADE,
    bio_kingdom_id INTEGER REFERENCES bio_kingdom_items(id) ON DELETE CASCADE,
    bio_phylum_id INTEGER REFERENCES bio_phylum_items(id) ON DELETE CASCADE,
    bio_class_id INTEGER REFERENCES bio_class_items(id) ON DELETE CASCADE
);