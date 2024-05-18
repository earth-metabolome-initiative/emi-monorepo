-- SQL defining the procedures table.
CREATE TABLE IF NOT EXISTS font_awesome_icons (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE, -- the name of the font awesome icon
    description TEXT NOT NULL -- a description of the font awesome icon
);