-- This is a no-op SQL statement
CREATE TABLE IF NOT EXISTS nameplate_categories (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    permanence_id INTEGER NOT NULL REFERENCES permanence_categories(id),
    material_id INTEGER NOT NULL REFERENCES materials(id),
    description TEXT NOT NULL,
    icon_id INTEGER NOT NULL REFERENCES font_awesome_icons(id),
    color_id INTEGER NOT NULL REFERENCES colors(id)
);