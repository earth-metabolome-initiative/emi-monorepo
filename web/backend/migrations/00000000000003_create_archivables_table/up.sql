-- SQL defining the archivables table, containing all archivable content.
CREATE TABLE archivables (
    id SERIAL PRIMARY KEY,
    editable_id INTEGER NOT NULL REFERENCES editables(id) ON DELETE CASCADE,
    archived_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    archived_by INTEGER NOT NULL REFERENCES users(id)
);