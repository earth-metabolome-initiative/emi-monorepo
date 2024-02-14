-- SQL to define the organization_users table.
-- A user appears in the organization_users table if they are a member of an organization.
-- The organization_users table is a many-to-many relation between users and organizations.
-- When either the user or the organization is deleted, the organization_users row should be deleted as well.
-- The role column is used to store the role of the user in the organization.
-- The created_at column is used to store the creation time of the record.
-- Since an administrator needs to add a user to an organization, the organization_users table
-- also contains a column to specify which administrator added the user to the organization.
CREATE TABLE organization_users (
    user_id int NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    organization_id int NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    role_id int NOT NULL REFERENCES organization_user_roles (id) ON DELETE SET NULL,
    editable_id int NOT NULL REFERENCES editables (id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, organization_id, role_id),
);