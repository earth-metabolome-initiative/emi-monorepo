-- SQL defining a state that a project may be in.
CREATE TABLE IF NOT EXISTS sample_states (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    font_awesome_icon_id INTEGER NOT NULL UNIQUE REFERENCES font_awesome_icons(id),
    color_id INTEGER NOT NULL UNIQUE REFERENCES colors(id)
);