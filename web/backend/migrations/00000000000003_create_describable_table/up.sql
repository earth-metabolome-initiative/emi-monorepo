-- Your SQL goes here
CREATE TABLE describable (
    id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT
);