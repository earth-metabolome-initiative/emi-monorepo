-- SQL defining a state that a project may be in.
CREATE TABLE IF NOT EXISTS bio_kingdom_items (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    ott_id INTEGER NOT NULL, -- cannot be null
    ncbi_id INTEGER, -- because it can be null
    gbif_id INTEGER, -- because it can be null
    bio_domain_id INTEGER REFERENCES bio_domain_items(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    font_awesome_icon TEXT NOT NULL,
    icon_color TEXT NOT NULL
);
