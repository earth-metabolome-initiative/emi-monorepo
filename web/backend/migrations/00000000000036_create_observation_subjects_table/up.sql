-- This is a no-op SQL statement
CREATE TABLE IF NOT EXISTS observation_subjects (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    icon_id INTEGER NOT NULL,
    color_id INTEGER NOT NULL,
    FOREIGN KEY (icon_id) REFERENCES font_awesome_icons(id),
    FOREIGN KEY (color_id) REFERENCES colors(id)
);