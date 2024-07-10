CREATE TABLE IF NOT EXISTS roles (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL UNIQUE,
    icon_id INTEGER UNIQUE NOT NULL,
    color_id INTEGER UNIQUE NOT NULL,
    FOREIGN KEY (icon_id) REFERENCES font_awesome_icons(id),
    FOREIGN KEY (color_id) REFERENCES colors(id)
);