-- This is a no-op SQL statement
CREATE TABLE IF NOT EXISTS sample_container_categories (
    id INTEGER PRIMARY KEY,
    brand TEXT NOT NULL,
    volume TEXT NOT NULL,
    description TEXT NOT NULL UNIQUE,
    icon_id INTEGER UNIQUE NOT NULL REFERENCES font_awesome_icons(id),
    color_id INTEGER UNIQUE NOT NULL REFERENCES colors(id)
);