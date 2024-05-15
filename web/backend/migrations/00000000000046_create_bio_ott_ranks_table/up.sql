-- SQL defining the procedures table.
CREATE TABLE IF NOT EXISTS bio_ott_ranks (
    id INTEGER PRIMARY KEY,
    -- the name of the taxon item
    name TEXT NOT NULL UNIQUE,
    -- the font awesome icon of the taxon item
    font_awesome_icon_id INTEGER NOT NULL REFERENCES font_awesome_icons(id) ON DELETE CASCADE
);