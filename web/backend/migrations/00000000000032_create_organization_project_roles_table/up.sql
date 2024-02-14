-- SQL defining the organization_projects_roles table.
-- An organization may have different roles in different projects.
CREATE TABLE organization_projects_roles (
    editable_id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT
);