CREATE TABLE IF NOT EXISTS organizations (
    id SERIAL PRIMARY KEY,
    parent_organization_id INTEGER DEFAULT NULL REFERENCES organizations(id) ON
    DELETE
        CASCADE,
        name TEXT NOT NULL
);