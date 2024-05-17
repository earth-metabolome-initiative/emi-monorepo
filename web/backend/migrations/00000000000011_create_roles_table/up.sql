CREATE TABLE IF NOT EXISTS roles (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL UNIQUE,
    font_awesome_icon_id INTEGER UNIQUE NOT NULL REFERENCES font_awesome_icons(id),
    color_id INTEGER UNIQUE NOT NULL REFERENCES colors(id)
);