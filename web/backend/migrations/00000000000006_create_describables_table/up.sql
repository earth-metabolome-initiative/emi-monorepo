-- Your SQL goes here
CREATE TABLE describables (
    id UUID PRIMARY KEY REFERENCES editables(id) ON
    DELETE
        CASCADE,
        name TEXT NOT NULL,
        description TEXT
);
