-- SQL defining a state that a project may be in.
CREATE TABLE IF NOT EXISTS bio_class_items (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    ott_id INTEGER NOT NULL, -- cannot be null
    ncbi_id INTEGER, -- because it can be null
    gbif_id INTEGER, -- because it can be null
    bio_domain_id INTEGER REFERENCES bio_domain_items(id) ON DELETE CASCADE,
    bio_kingdom_id INTEGER REFERENCES bio_kingdom_items(id) ON DELETE CASCADE,
    bio_phylum_id INTEGER REFERENCES bio_phylum_items(id) ON DELETE CASCADE
);
