-- This is a no-op SQL statement
CREATE TABLE IF NOT EXISTS nameplate_categories (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    permanence_id INTEGER NOT NULL,
    material_id INTEGER NOT NULL,
    description TEXT NOT NULL,
    icon_id INTEGER NOT NULL,
    color_id INTEGER NOT NULL,
    FOREIGN KEY (permanence_id) REFERENCES permanence_categories(id),
    FOREIGN KEY (material_id) REFERENCES materials(id),
    FOREIGN KEY (icon_id) REFERENCES font_awesome_icons(id),
    FOREIGN KEY (color_id) REFERENCES colors(id)
);