-- This is a no-op SQL statement
CREATE TABLE IF NOT EXISTS materials(
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    icon_id INTEGER REFERENCES font_awesome_icons(id),
    color_id INTEGER REFERENCES colors(id)
);