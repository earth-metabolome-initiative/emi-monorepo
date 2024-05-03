-- SQL defining the procedures table.
CREATE TABLE IF NOT EXISTS sampling_procedures (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL
);