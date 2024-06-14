-- This is a no-op SQL statement
CREATE TABLE IF NOT EXISTS sample_container_categories (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    volume FLOAT NOT NULL,
    unit TEXT NOT NULL,
    material_id INTEGER NOT NULL,
    description TEXT NOT NULL,
    icon_id INTEGER NOT NULL,
    color_id INTEGER NOT NULL,
    FOREIGN KEY (material_id) REFERENCES materials(id),
    FOREIGN KEY (icon_id) REFERENCES font_awesome_icons(id),
    FOREIGN KEY (color_id) REFERENCES colors(id)
);