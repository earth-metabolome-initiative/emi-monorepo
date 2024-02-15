-- SQL defining the organization_project_roles table.
-- An organization may have different roles in different projects.
CREATE TABLE organization_project_roles (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describables(id) ON DELETE CASCADE
);