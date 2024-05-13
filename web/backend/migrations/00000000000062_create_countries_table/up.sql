-- This is a no-op SQL statement
CREATE TABLE IF NOT EXISTS countries (
    id INTEGER PRIMARY KEY,
    ISO TEXT NOT NULL UNIQUE,
    emoji TEXT NOT NULL UNIQUE,
    unicode TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL UNIQUE
);