-- SQL to define the organization_users table.
-- A user appears in the organization_users table if they are a member of an organization.
-- The organization_users table is a many-to-many relation between users and organizations.
-- When either the user or the organization is deleted, the organization_users row should be deleted as well.
-- The role column is used to store the role of the user in the organization.
-- The created_at column is used to store the creation time of the record.
-- Since an administrator needs to add a user to an organization, the organization_users table
-- also contains a column to specify which administrator added the user to the organization.
CREATE TABLE organization_users (
    user_id int NOT NULL,
    organization_id int NOT NULL,
    added_by int NOT NULL,
    role int NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, organization_id, role),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id) REFERENCES organizations (id) ON DELETE CASCADE,
    FOREIGN KEY (added_by) REFERENCES users (id) ON DELETE CASCADE
);