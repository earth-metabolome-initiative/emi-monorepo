-- This is a no-op SQL statement
CREATE TABLE colors (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE, -- the name of the color
    hexadecimal_value TEXT NOT NULL UNIQUE -- the hex code of the color
);