-- SQL defining the archivables table, containing all archivable content.
CREATE TABLE archivables (
    id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    archived_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    archived_by INTEGER NOT NULL REFERENCES users(id)
);