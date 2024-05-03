CREATE TABLE IF NOT EXISTS organizations (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    parent_organization_id INTEGER DEFAULT NULL REFERENCES organizations(id) ON
    DELETE
        CASCADE,
        name TEXT NOT NULL
);