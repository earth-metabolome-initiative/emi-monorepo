CREATE TABLE IF NOT EXISTS units (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    unit TEXT NOT NULL,
    icon_id INTEGER NOT NULL,
    color_id INTEGER NOT NULL,
    FOREIGN KEY (icon_id) REFERENCES font_awesome_icons(id),
    FOREIGN KEY (color_id) REFERENCES colors(id)
);
