-- This is a no-op SQL statement
CREATE TABLE IF NOT EXISTS observation_subjects (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    parent_observation_subject_id INTEGER REFERENCES observation_subjects(id),
    description TEXT NOT NULL,
    icon_id INTEGER NOT NULL REFERENCES font_awesome_icons(id),
    color_id INTEGER NOT NULL REFERENCES colors(id)
);