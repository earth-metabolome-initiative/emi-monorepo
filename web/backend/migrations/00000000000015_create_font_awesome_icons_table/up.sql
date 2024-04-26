-- SQL defining the procedures table.
CREATE TABLE font_awesome_icons (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE -- the name of the font awesome icon
);