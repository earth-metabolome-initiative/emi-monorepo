-- SQL defining the archivables table, containing all archivable content.
CREATE TABLE archivables (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON
    DELETE
        CASCADE,
        archived_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        archived_by INTEGER NOT NULL REFERENCES users(id)
);

-- We DO NOT add a trigger to delete the corresponding record in the editables table when an archivable is deleted.
-- This is because, while an editable can be archived, it can also exist in other states.
