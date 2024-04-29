-- SQL defining a state that a team may be in.
CREATE TABLE IF NOT EXISTS team_states (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    font_awesome_icon TEXT NOT NULL,
    icon_color TEXT NOT NULL
);
