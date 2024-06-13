-- SQL defining a state that a project may be in.
CREATE TABLE IF NOT EXISTS project_states (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    icon_id INTEGER NOT NULL UNIQUE,
    color_id INTEGER NOT NULL UNIQUE,
    FOREIGN KEY (icon_id) REFERENCES font_awesome_icons(id),
    FOREIGN KEY (color_id) REFERENCES colors(id)
);