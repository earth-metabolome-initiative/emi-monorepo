CREATE TABLE IF NOT EXISTS colors (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE, -- the name of the color
    hexadecimal_value TEXT NOT NULL UNIQUE, -- the hex code of the color
    description TEXT NOT NULL -- a description of the color
);