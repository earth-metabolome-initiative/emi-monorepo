-- SQL defining the organization_project_roles table.
-- An organization may have different roles in different projects.
CREATE TABLE organization_project_roles (
    id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT
);