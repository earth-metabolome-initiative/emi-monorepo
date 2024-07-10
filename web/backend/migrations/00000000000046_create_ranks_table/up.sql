-- SQL defining the procedures table.
CREATE TABLE IF NOT EXISTS ranks (
    id INTEGER PRIMARY KEY,
    -- the name of the taxon item
    name TEXT NOT NULL UNIQUE,
    -- the description of the taxon item
    description TEXT NOT NULL,
    -- the font awesome icon of the rank
    icon_id INTEGER NOT NULL,
    -- the color associated with the rank
    color_id INTEGER NOT NULL,
    FOREIGN KEY (icon_id) REFERENCES font_awesome_icons(id) ON DELETE CASCADE,
    FOREIGN KEY (color_id) REFERENCES colors(id) ON DELETE CASCADE
);