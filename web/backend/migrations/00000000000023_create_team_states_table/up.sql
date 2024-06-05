-- SQL defining a state that a team may be in.
CREATE TABLE IF NOT EXISTS team_states (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    icon_id INTEGER NOT NULL UNIQUE REFERENCES font_awesome_icons(id) ON DELETE CASCADE,
    color_id INTEGER NOT NULL UNIQUE REFERENCES colors(id) ON DELETE CASCADE
);