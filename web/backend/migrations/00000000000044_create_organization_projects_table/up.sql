-- SQL defining the organization_projects table.
-- This table defines a link from an organization to a project, with a viewer role.
-- All users that are part of the organization will be able to view the project.
-- The organization_id and project_id columns are used to store the organization and project,
-- which are used as primary keys. The created_at column is used to store the creation time of the record.
-- Since only an organization administrator can add link a project to an organization,
-- the organization_projects table also contains a column to specify which administrator
-- added the project to the organization.
CREATE TABLE organization_projects (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    organization_id BIGINT NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    project_id BIGINT NOT NULL REFERENCES projects (id) ON DELETE CASCADE,
    role_id BIGINT NOT NULL REFERENCES team_user_roles (id) ON DELETE CASCADE
);
