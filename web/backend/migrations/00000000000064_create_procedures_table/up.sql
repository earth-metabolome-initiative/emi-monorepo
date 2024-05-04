-- SQL defining the procedures table.
CREATE TABLE IF NOT EXISTS procedures (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE
);