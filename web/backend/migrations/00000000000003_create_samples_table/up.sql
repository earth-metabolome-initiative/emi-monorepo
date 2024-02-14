-- UP MIGRATION
CREATE TABLE samples (
    id SERIAL PRIMARY KEY,
    derived_from INTEGER REFERENCES samples(id) ON DELETE SET NULL,
    editable_id INTEGER NOT NULL REFERENCES editables(id),
);