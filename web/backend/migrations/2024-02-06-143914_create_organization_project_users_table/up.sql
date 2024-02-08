-- SQL defining the organization_project_users table.
-- This table defines a link from an organization to a project, with a viewer role.
-- All users that are part of the organization will be able to view the project.
-- The organization_id and project_id columns are used to store the organization and project,
-- which are used as primary keys. The created_at column is used to store the creation time of the record.
-- Since only an organization administrator can add link a project to an organization,
-- the organization_project_users table also contains a column to specify which administrator
-- added the project to the organization.
CREATE TABLE organization_project_users (
    organization_id int NOT NULL,
    project_id int NOT NULL,
    added_by int NOT NULL,
    role int NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (organization_id, project_id, role),
    FOREIGN KEY (organization_id) REFERENCES organizations (id) ON DELETE CASCADE,
    FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE,
    FOREIGN KEY (added_by) REFERENCES users (id) ON DELETE CASCADE
);
